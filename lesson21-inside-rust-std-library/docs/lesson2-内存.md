# RUST标准库内存模块代码分析
RUST最难以掌握特性之一是RUST的内存操作。RUST与C相同，需要对内存做彻底的控制，即程序可以在代码中编写专属内存管理系统，并将内存管理系统与语言类型相关联，在内存块与语言类型之间做自如的转换。对于当前现代语法的高级语言如Java/Python/JS/Go，内存管理是编译器的任务，这就导致大部分程序员对于内存管理缺乏经验，所以对RUST内存安全相关的所有权/生命周期等缺乏实践认知。
相对于C，RUST的现代语法及内存安全语法导致RUST的内存块与类型系统相互转换的细节非常复杂，不容易被透彻理解。本章将从标准库的内存模块的代码分析中给出RUST内存操作的本质。对本章内容掌握了，RUST语言的最难关便过了。


从内存角度考察一个变量，则每个变量具备统一的内存参数，这些参数是：
1. 变量的首地址，是一个usize的数值
2. 变量类型占用的内存块大小
3. 变量类型内存字节对齐的基数
4. 变量类型中成员内存顺序

如果变量成员是复合类型，可递归上面的四个参数。
RUST认为变量类型成员顺序与编译优化不可分割，因此，变量成员内存顺序完全由编译器控制
RUST具备将一块内存块直接转换成某一类型变量的能力, 这一能力是RUST系统级编程及高性能的一个基石。但因为这个转换使得代码可以绕过编译器的类型系统检查，造成了BUG也绕过了编译器的某些错误检查，而这些错误很可能在系统运行很久之后才真正的出错，造成排错的极高成本。

掌握RUST的内存，主要有如下几个部分：
1. 编译器提供的固有内存操作函数
2. 内存块与类型系统的结合点：裸指针 `*const T/*mut T`
3. 裸指针的包装结构: `NonNull<T>/Unique<T>`
4. 未初始化内存块的处理：`MaybeUninit<T>/ManuallyDrop<T>`
5. 堆内存申请及释放

## 裸指针标准库代码分析
裸指针`*const T/* mut T`将内存和类型系统相连接，裸指针代表了一个内存块，指示了内存块首地址，大小，对齐等属性及后文提到的元数据，但不保证这个内存块的有效性和安全性。
与`*const T/* mut T`不同，`&T/&mut T`则保证内存块是安全和有效的，即`&T/&mut T`满足内存块首地址内存对齐，内存块已经完成了初始化。在RUST中，`&T/&mut T`是被绑定在某一内存块上，只能用于读写这一内存块。

对于内存块更复杂的操作，由`*const T/*mut T` 负责，主要有：
1. 将`usize`类型数值强制转换成裸指针类型，以此数值为首地址的内存块被转换为相应的类型。如果对这一转换后的内存块进行读写，可能造成内存安全问题。
2. 在不同的裸指针类型之间进行强制转换，实质上完成了裸指针指向的内存块的类型强转，如果对这一转换后的内存块进行读写，可能造成内存安全问题。
3. `*const u8`作为堆内存申请的内存块绑定变量
4. 内存块置值操作，如清零或置一个魔术值
5. 显式的内存块拷贝操作，某些情况下，内存块拷贝是必须的高性能方式。
6. 利用指针偏移计算获取新的内存块， 在数组及切片访问，字符串，协议字节填写，文件缓存等都需要指针偏移计算。
7. 从外部的C函数接口对接的指针参数
8. ...

RUST的裸指针类型不象C语言的指针类型那样仅仅是一个地址值，为满足实现内存安全的类型系统需求，并兼顾内存使用效率和方便性，RUST的裸指针实质是一个较复杂的类型结构体。

### 裸指针具体实现
`*const T/*mut T`实质是个数据结构体，由两个部分组成，第一个部分是一个内存地址，第二个部分对这个内存地址的约束性描述-元数据
```rs
//从下面结构定义可以看到，裸指针本质就是PtrComponents<T>
pub(crate) union PtrRepr<T: ?Sized> {
    pub(crate) const_ptr: *const T,
    pub(crate) mut_ptr: *mut T,
    pub(crate) components: PtrComponents<T>,
}

pub(crate) struct PtrComponents<T: ?Sized> {
    //*const ()保证元数据部分是空 
    pub(crate) data_address: *const (),
    //不同类型指针的元数据
    pub(crate) metadata: <T as Pointee>::Metadata,
}

//下面Pointee的定义展示一个RUST的编程技巧，即trait可以只用
//来定义关联类型，Pointee即只用来指定Metadata的类型。
pub trait Pointee {
    /// The type for metadata in pointers and references to `Self`.
    type Metadata: Copy + Send + Sync + Ord + Hash + Unpin;
}
//廋指针元数据是单元类型，即是空
pub trait Thin = Pointee<Metadata = ()>;
```

元数据的规则:
- 对于固定大小类型的指针（实现了 `Sized Trait`）, RUST定义为廋指针(`thin pointer`)，元数据大小为`0`，类型为`()`,这里要注意，RUST中数组也是固定大小的类型，运行中对数组下标合法性的检测，就是比较是否已经越过了数组的内存大小。
- 对于动态大小类型的指针(`DST` 类型)，RUST定义为胖指针(`fat pointer` 或 `wide pointer`), 元数据为：
  - 对于结构类型，如果最后一个成员是动态大小类型(结构的其他成员不允许为动态大小类型)，则元数据为此动态大小类型
的元数据
  - 对于`str`类型, 元数据是按字节计算的长度值，元数据类型是`usize`
  - 对于切片类型，例如`[T]`类型，元数据是数组元素的数目值，元数据类型是`usize`
  - 对于`trait`对象，例如 `dyn SomeTrait`， 元数据是 `[DynMetadata][DynMetadata]`（后面代码解释） （例如：`DynMetadata`) 随着RUST的发展，有可能会根据需要引入新的元数据种类。


在标准库代码当中没有指针类型如何实现`Pointee Trait`的代码，编译器针对每个类型自动的实现了`Pointee`。 如下为rust编译器代码的一个摘录
```rs
    pub fn ptr_metadata_ty(&'tcx self, tcx: TyCtxt<'tcx>) -> Ty<'tcx> {
        // FIXME: should this normalize?
        let tail = tcx.struct_tail_without_normalization(self);
        match tail.kind() {
            // Sized types
            ty::Infer(ty::IntVar(_) | ty::FloatVar(_))
            | ty::Uint(_)
            | ty::Int(_)
            | ty::Bool
            | ty::Float(_)
            | ty::FnDef(..)
            | ty::FnPtr(_)
            | ty::RawPtr(..)
            | ty::Char
            | ty::Ref(..)
            | ty::Generator(..)
            | ty::GeneratorWitness(..)
            | ty::Array(..)
            | ty::Closure(..)
            | ty::Never
            | ty::Error(_)
            | ty::Foreign(..)
            | ty::Adt(..)
            // 如果是固定类型，元数据是单元类型 tcx.types.unit，即为空
            | ty::Tuple(..) => tcx.types.unit,

            //对于字符串和切片类型，元数据为长度tcx.types.usize，是元素长度
            ty::Str | ty::Slice(_) => tcx.types.usize,

            //对于dyn Trait类型， 元数据从具体的DynMetadata获取*
            ty::Dynamic(..) => {
                let dyn_metadata = tcx.lang_items().dyn_metadata().unwrap();
                tcx.type_of(dyn_metadata).subst(tcx, &[tail.into()])
            },
            
            //以下类型不应有元数据
            ty::Projection(_)
            | ty::Param(_)
            | ty::Opaque(..)
            | ty::Infer(ty::TyVar(_))
            | ty::Bound(..)
            | ty::Placeholder(..)
            | ty::Infer(ty::FreshTy(_) | ty::FreshIntTy(_) | ty::FreshFloatTy(_)) => {
                bug!("`ptr_metadata_ty` applied to unexpected type: {:?}", tail)
            }
        }
    }
```
对于trait对象的元数据的具体结构定义见如下代码：
```rs
//dyn trait裸指针的元数据结构,此元数据会被用于获取trait的方法
pub struct DynMetadata<Dyn: ?Sized> {
    //在堆内存中的VTTable变量的引用,VTable见后面的说明
    vtable_ptr: &'static VTable,
    //标示结构对Dyn的所有权关系，
    //其中PhantomData与具体变量的联系在初始化时由编译器自行推断完成, 
    //这里PhantomData主要对编译器提示做Drop check时注意本结构体会
    //负责对Dyn类型变量做drop。
    phantom: crate::marker::PhantomData<Dyn>,
}

//此结构是实际的trait实现
struct VTable {
    //trait对象的drop方法的指针
    drop_in_place: fn(*mut ()),
    //trait对象的内存大小
    size_of: usize,
    //trait对象的内存对齐
    align_of: usize,
    //后继是trait对象的所有方法的指针数组
}
```

元数据类型相同的裸指针可以任意的转换，例如：可以有 `* const [usize; 3] as * const[usize; 5]` 这种语句.
元数据类型不同的裸指针之间不能转换，例如；`* const [usize;3] as *const[usize]` 这种语句无法通过编译器

#### 裸指针的操作函数——intrinsic模块内存相关固有函数
`intrinsics`模块中的函数由编译器内置实现，并提供给其他模块使用。固有函数标准库没有代码，所以对其主要是了解功能和如何使用，`intrinsics`模块的内存函数一般不由库以外的代码直接调用，而是由`mem`模块和`ptr`模块封装后再提供给其他模块。
内存申请及释放函数：
`intrinsics::forget<T:Sized?> (_:T)`, 代码中调用这个函数后，在变量生命周期终止时，编译器不会调用变量的drop函数
`intrinsics::drop_in_place<T:Sized?>(to_drop: * mut T)` 在`forget`后，如果仍然需要对变量调用`drop`，则在代码中显式调用此函数以触发对变量的`drop`调用。
`intrinsics::needs_drop<T>()->bool`, 判断`T`类型是否需要做`drop`操作，实现了`Copy trait`的类型会返回`false`

类型转换：
`intrinsics::transmute<T,U>(e:T)->U`, 对于内存布局相同的类型 `T`和`U`, 完成将类型`T`变量转换为类型`U`变量，此时`T`的所有权将转换为`U`的所有权

指针偏移函数: 
`intrinsics::offset<T>(dst: *const T, offset: usize)->* const T`, 相当于C的基于类型的指针加计算
`intrinsics::ptr_offset_from<T>(ptr: *const T, base: *const T) -> isize` 相当于C的基于类型的指针减

内存块内容修改函数: 
`intrinsics::copy<T>(src:*const T, dst: *mut T, count:usize)`, 内存拷贝， `src`和`dst`内存可重叠， 类似c语言中的`memmove`, 此时`dst`原有内存如果已经初始化，`dst`原有变量的`drop`实质会不执行。`src`的变量可能出现两次`drop`，因此调用此函数的代码需要处理这种情况。
`intrinsics::copy_no_overlapping<T>(src:*const T, dst: * mut T, count:usize)`, 内存拷贝， `src`和`dst`内存不重叠，内存安全问题同上

> 这两个函数之间的区别在于内存重叠的处理方式：
  `intrinsics::copy<T>(src: *const T, dst: *mut T, count: usize)` 函数用于将 `count` 个元素从 `src` 复制到 `dst`，并且允许源和目标内存块有重叠部分。这意味着如果 `src` 和 `dst` 指向的内存区域有重叠，复制的结果可能是不确定的。
  `intrinsics::copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)` 函数与前者类似，用于将 `count` 个元素从 `src` 复制到 `dst`，但要求源和目标内存块是不重叠的。如果源和目标内存块有重叠部分，这个函数的行为是未定义的，可能导致意外的结果。

`intrinsics::write_bytes(dst: *mut T, val:u8, count:usize)` , C语言的`memset`的RUST实现, 此时，原内存如果已经初始化，则因为编译器会继续对`dst`的内存块做`drop`调用，有可能会Undefined Behavior。

类型内存参数函数：
`intrinsics::size_of<T>()->usize` 类型内存空间字节数
`intrinsics::min_align_of<T>()->usize` 返回类型对齐字节数
`intrinsics::size_of_val<T>(_:*const T)->usize` 返回指针指向的变量内存空间字节数
`intrinsics::min_align_of_val<T>(_: * const T)->usize` 返回指针指向的变量对齐字节数

禁止优化的内存函数： 形如`volatile_xxxx` 的函数是通知编译器不做内存优化的操作函数,一般硬件相关操作需要禁止优化。
`intrinsics::volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, count: usize)` 内存拷贝
`intrinsics::volatile_copy_memory<T>(dst: *mut T, src: *const T, count: usize)` 功能类似C语言memmove
`intrinsics::volatile_set_memory<T>(dst: *mut T, val: u8, count: usize)` 功能类似C语言memset
`intrinsics::volatile_load<T>(src: *const T) -> T` 读取内存或寄存器，T类型字节对齐到2的幂次
`intrinsics::volatile_store<T>(dst: *mut T, val: T)`内存或寄存器写入，字节对齐
`intrinsics::unaligned_volatile_load<T>(src: *const T) -> T` 字节非对齐
`intrinsics::unaligned_volatile_store<T>(dst: *mut T, val: T)`字节非对齐

内存比较函数： 
`intrinsics::raw_eq<T>(a: &T, b: &T) -> bool` 内存比较，类似C语言memcmp
`pub fn ptr_guaranteed_eq<T>(ptr: *const T, other: *const T) -> bool` 判断两个指针是否相等, 相等返回ture, 不等返回false
`pub fn ptr_guaranteed_ne<T>(ptr: *const T, other: *const T) -> bool` 判断两个指针是否不等，不等返回true



#### 裸指针方法
对于裸指针，RUST标准库包含了最基础的 `* const T/* mut T`， 以及在`* const T/*mut T` 基础上特化的切片类型`[T]`的裸指针`* const [T]/*mut [T]`。 标准库针对这两种类型实现了一些关联函数及方法。这里一定注意，所有针对 `* const T`的方法在`* const [T]`上都是适用的。

##### 裸指针的创建
直接从已经初始化的变量创建裸指针：
```rs
    &T as *const T;
    &mut T as * mut T;
```
直接用usize的数值创建裸指针：
```rs
    {
        let  a: usize = 0xf000000000000000;
        unsafe {a as * const i32};
    }
```

操作系统内核经常需要直接将一个地址数值转换为某一类型的裸指针
RUST也提供了一些其他的裸指针创建关联函数：
`ptr::null<T>() -> *const T` 创建一个`0`值的`*const T`，实际上就是 `0 as *const T`，用`null()`函数明显更符合程序员的习惯
`ptr::null_mut<T>()->*mut T` 除了类型以外，其他同上

将指针变量用作他途以提高性能：
`ptr::invalid<T>(addr:usize)->*mut T` 将一个数值作为裸指针，指明这是一个无效的裸指针。
`ptr::invalid_mut<T>(addr:usize)->*mut T` 将一个数值作为可变裸指针，指明这是一个无效的指针。

RUST裸指针类型转换时，经常使用以下两个函数获得需要的指针类型:
`ptr::from_raw_parts<T: ?Sized>(data_address: *const (), metadata: <T as Pointee>::Metadata) -> *const T` 从内存地址和元数据创建裸指针
`ptr::from_raw_parts_mut<T: ?Sized>(data_address: *mut (), metadata: <T as Pointee>::Metadata) -> *mut T` 功能同上，创建可变裸指针

切片类型的裸指针创建函数如下：
`ptr::slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T]` 
`ptr::slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T]` 由裸指针类型及切片长度获得切片类型裸指针，调用代码应保证`data`事实上就是切片的裸指针地址。

由类型裸指针转换为切片类型裸指针最突出的应用之一是内存申请，申请的内存返回 `* const u8`的指针，这个裸指针是没有包含内存大小的，只有头地址，因此需要将这个指针转换为 `* const [u8]`，将申请的内存大小包含入裸指针结构体中。

`slice_from_raw_parts`代码如下：
```rs
pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
    //data.cast()将*const T转换为 *const()
    from_raw_parts(data.cast(), len)
}

pub const fn from_raw_parts<T: ?Sized>(
    data_address: *const (),
    metadata: <T as Pointee>::Metadata,
) -> *const T {
    //由以下代码可以确认 * const T实质就是PtrRepr类型结构体。
    unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.const_ptr }
}
```

##### 裸指针类型转换方法






