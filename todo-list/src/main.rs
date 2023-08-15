use std::{error::Error, fs::OpenOptions};

use chrono::{DateTime, Local, TimeZone, Timelike, Utc};
use clap::{Args, Parser, Subcommand, ValueEnum};
use csv::{Reader, StringRecord, Writer, WriterBuilder};
use prettytable::{row, Cell, Row, Table};
use serde::{Deserialize, Serialize};

static DATA_FILE_PATH: &str = "./src/data.csv";

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    id: i32,
    title: String,
    done: bool,
    date_time: i64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    List {
        #[arg(default_value_t =ListActionType::NotDone)]
        action: ListActionType,
    },
    Add {
        title: String,
    },
    Remove {
        id: i32,
    },
    Update {
        id: i32,
        title: String,
    },
    Toggle {
        id: i32
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum ListActionType {
    All,
    Done,
    NotDone,
}

impl ToString for ListActionType {
    fn to_string(&self) -> String {
        match *self {
            ListActionType::All => String::from("all"),
            ListActionType::Done => String::from("done"),
            ListActionType::NotDone => String::from("not-done"),
        }
    }
}

fn main() -> Result<(), csv::Error> {
    let args = Cli::parse();
    match &args.cmd {
        Cmd::List { action } => display_list(action),
        Cmd::Add { title } => add_item(title),
        Cmd::Remove { id } => remove_item(id),
        Cmd::Update { id, title } => todo!(),
        Cmd::Toggle { id } => toggle(id),
    }
}

fn display_list(action: &ListActionType) -> Result<(), csv::Error> {
    let mut records = get_records()?;
    let mut table = Table::new();

    records = match action {
        ListActionType::All => records,
        ListActionType::Done => records.into_iter().filter(|record| record[2] == "1".to_string()).collect::<Vec<Vec<String>>>(),
        ListActionType::NotDone => records.into_iter().filter(|record| record[2] == "0".to_string()).collect::<Vec<Vec<String>>>(),
    };

    records.insert(0, vec!["id".to_string(), "title".to_string(), "done".to_string(), "time".to_string()]);

    for record in records {
        let cells: Vec<Cell> = record.iter().map(|x| Cell::new(&x.to_string())).collect();
        table.add_row(Row::new(cells));
    }
    table.printstd();
    Ok(())
}

fn add_item(title: &str) -> Result<(), csv::Error> {
    let this_id = get_last_id()? + 1;

    let file = OpenOptions::new().append(true).open(DATA_FILE_PATH)?;
    let mut writer = WriterBuilder::new().from_writer(file);

    let new_row = [
        &this_id.to_string(),
        title,
        "0",
        &Utc::now().timestamp().to_string(),
    ];

    writer.write_record(new_row)?;
    writer.flush()?;

    display_list(&ListActionType::All)?;

    Ok(())
}

fn get_last_id() -> Result<i32, csv::Error> {
    let records = get_records()?;

    if records.len() > 0 {
        let last_id = records.last().unwrap()[0].parse::<i32>().unwrap();
        return Ok(last_id);
    }

    Ok(1)
}

fn remove_item(id: &i32) -> Result<(), csv::Error> {
    let mut records = get_records()?;
    records = records.into_iter().filter(|record| {record[0] != id.to_string()}).collect::<Vec<Vec<String>>>();
    
    write_records(records)?;

    display_list(&ListActionType::All)?;

    Ok(())
}


fn get_records() -> Result<Vec<Vec<String>>, csv::Error> {
    let mut rdr = Reader::from_path(DATA_FILE_PATH)?;
    let mut records = vec![];
    // let header = rdr.headers()?;
    // records.push(header.iter().map(|i| i.to_string()).collect::<Vec<String>>());

    for item in rdr.records() {
        let record = item?;
        records.push(record.iter().map(|i| i.to_string()).collect::<Vec<String>>());
    }

    Ok(records)
}

fn write_records(mut records: Vec<Vec<String>>) -> Result<(), csv::Error> {
    let mut open_options = OpenOptions::new();
    open_options.write(true).truncate(true); // 覆盖已存在的文件
    
    let mut writer = WriterBuilder::new().from_writer(open_options.open(DATA_FILE_PATH)?);

    records.insert(0, vec!["id".to_string(), "title".to_string(), "done".to_string(), "time".to_string()]);

    for item in records {
        writer.write_record(item)?;
    }

    writer.flush()?;
    Ok(())
}

fn toggle(id: &i32) -> Result<(), csv::Error> {
    let mut records = get_records()?;
    for record in &mut records {
        if record[0] == id.to_string() {
            record[2] = ((record[2].parse::<i32>().unwrap()) ^ 1).to_string();
            break;
        }
    }

    write_records(records)?;

    display_list(&ListActionType::All)?;

    Ok(())
}

#[test]
fn test() {
    let a = 0;
    println!("{}", a ^ 1);
}