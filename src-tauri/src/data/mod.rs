
use calamine::{open_workbook,Reader, Xlsx};

use rusqlite::{Connection, Result, params};
use serde::ser::{Serialize, Serializer, SerializeStruct};

//BASE functions
fn create_connection()->Result<Connection>{
    let conn = Connection::open("data.db")?;
    Ok(conn)
}
//CREATE PARTS
fn add_photographers(excel_path: &str, conn: &mut Connection)->Result<()>{
    let mut workbook: Xlsx<_> = open_workbook(excel_path).unwrap();
    // Create a vector to store the data
    let mut photographers = Vec::new();
    println!("open path:{}", excel_path);

    if let Some(Ok(range)) = workbook.worksheet_range("photographers") {
        let mut count = 0;
        photographers.push((0, String::from("NOT SET")));
        for row in range.rows() {
            //println!("row={:?}, row[0]={:?}", row, row[0]);
            if count > 0{
                let id = row[0].get_float().unwrap_or(0.0) as i32;
                let name = row[1].get_string().unwrap_or("").to_owned();
            
                println!("id: {}, name: {}", id, name);
                photographers.push((id, name));
        }
        count += 1;
        }
    }
    else{
        println!("Cannot find photographers sheet!")
    }

    if photographers.len() > 0{

        println!("Prepare connection...");

        let create_photographers_sql = "CREATE TABLE IF NOT EXISTS 'photographers (
            'id' INTEGER PRIMARY KEY,
            'name' TEXT NOT NULL
        );";

        conn.execute(create_photographers_sql, params![])?;

        let tx = conn.transaction()?;
        println!("Prepare Transaction...");
        
        for (id, name) in photographers{
            //println!("Excuting for ({}, {})", id, &name);
            match tx.execute("INSERT INTO photographers (id, name) VALUES (?1, ?2)", params![id,&name]){
                Ok(updated) => continue,
                Err(err) => println!("Insert photographer failed id:{}, err: {}",id, err),
            };
            
        }
        tx.commit();
    }
    Ok(())
}


fn add_afu(excel_path: &str, conn: &mut Connection)->Result<()>{
    let mut workbook: Xlsx<_> = open_workbook(excel_path).unwrap();
    // Create a vector to store the data
    let mut afus = Vec::new();
    println!("open path:{}", excel_path);

    if let Some(Ok(range)) = workbook.worksheet_range("afu") {
        let mut count = 0;
        for row in range.rows() {
            //println!("row={:?}, row[0]={:?}", row, row[0]);
            if count > 0{
                let community = row[0].get_string().unwrap_or("").to_owned();
                let school = row[1].get_string().unwrap_or("").to_owned();
                let child_id = row[2].get_float().unwrap_or(0.0) as i32;
                let child_name = row[3].get_string().unwrap_or("").to_owned();
                let sex = row[4].get_string().unwrap_or("").to_owned();
                let last_grade = row[5].get_string().unwrap_or(&row[5].get_float().unwrap_or(0.0).to_string()).to_owned();
                if last_grade== String::from("") && count< 5{
                    println!("last grade CELL: {:?}", row[5]);
                }
                
                let last_status = row[6].get_string().unwrap_or("Ineligible").to_owned();
                let pg_id = row[7].get_float().unwrap_or(0.0) as i32;

            
                //println!("{}. {}, {}, {}, {}, {}, {}, {}, {}", count, community, school, child_id, child_name, sex, last_grade, last_status, pg_id);
                //follow table format
                afus.push((child_id, child_name, sex, last_grade, last_status, school, community, pg_id));
        }
        count += 1;
        }
    }
    if afus.len() > 0{
        let create_afu_sql = "CREATE TABLE IF NOT EXISTS 'afu' (
            'id' INTEGER PRIMARY KEY AUTOINCREMENT,
            'child_id' INTEGER UNIQUE NOT NULL,
            'child_name' TEXT NOT NULL,
            'sex' TEXT NOT NULL,
            'last_grade' TEXT NOT NULL,
            'last_status' TEXT NOT NULL,
            'school' TEXT NOT NULL,
            'community' TEXT NOT NULL,
            'pg_id' INTEGER DEFAULT 0,
            'smile_score' INTEGER DEFAULT 0,
            'bg_score' INTEGER DEFAULT 0,
            'clarity_score' INTEGER DEFAULT 0,
            FOREIGN KEY ('pg_id') REFERENCES 'photographers'('id') ON DELETE SET NULL)";

        conn.execute(create_afu_sql, params![])?;
        let tx = conn.transaction()?;
        println!("Prepare AFU Transaction...");

        for (child_id, child_name, sex, last_grade, last_status, school, community, pg_id) in afus{
            //println!("Excuting for ({}, {})", child_id, &child_name);
            let sql_insert_afu = "INSERT INTO afu (child_id, child_name, sex, last_grade, last_status, school, community, pg_id) VALUES 
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
            match tx.execute(sql_insert_afu, params![child_id,&child_name,&sex,&last_grade, &last_status, &school, &community, pg_id]){
                Ok(updated) => continue,
                Err(err) => println!("Insert AFU failed! child id:{}, pg_id:{}, err: {}", child_id, pg_id, err),
            };
        } 
        tx.commit();   
    }
    Ok(())    
}

pub fn import_excel_to_sqlite(excel_path: &str)->Result<()>{
    let mut conn = Connection::open("data.db")?;

    if let Ok(trs) = add_photographers(excel_path, &mut conn){
        
        if let Ok(trs2) = add_afu(excel_path, &mut conn){
           println!("Success run whole import: {:?}, {:?}", &trs, &trs2);
        }
        else{
           println!("Adding afu failed");
        }
    }
    else {
        println!("Adding photographers failed");
    }
    Ok(())
}

//READ PARTS
pub struct Photographer{
    pub id: i32,
    pub name: String
}
impl Serialize for Photographer{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {   
        // 1. Serilize the struct
        // 11 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Photographer", 2)?;
        //2. serialize_field
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

pub struct Stat{
    pub school: String,
    pub num_elig: i32,
    pub num_inelig: i32
}
impl Serialize for Stat{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {   
        // 1. Serilize the struct
        // 11 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Stat", 3)?;
        //2. serialize_field
        state.serialize_field("school", &self.school)?;
        state.serialize_field("num_elig", &self.num_elig)?;
        state.serialize_field("num_inelig", &self.num_inelig)?;
        state.end()
    }
}
//Get all statistics of a photographer
pub fn get_pg_school_stats(pg_id: i32, school:&str)->Result<Stat>{
    
    let conn = Connection::open("data.db")?;
    let stats_sql = "SELECT 
                                school,
                                COUNT(CASE WHEN last_status = 'Eligible' THEN child_id ELSE NULL END) AS eligible,
                                COUNT(CASE WHEN last_status = 'Ineligible' THEN child_id ELSE NULL END) AS ineligible
                            FROM afu
                            WHERE pg_id = ?1 and school = ?2
                            group by school";
    let mut stmt = conn.prepare(stats_sql)?;
    let mut rows = stmt.query(params![pg_id, &school])?;
    
    

    while let Some(row) = rows.next().expect("No rows") {
       let stat = Stat {
            school: row.get(0)?,
            num_elig: row.get(1)?,
            num_inelig: row.get(2)?
       };
       println!("Req Stats: {}, {}, {}", stat.school, stat.num_elig, stat.num_inelig);
       return Ok(stat);
    }
    
    Ok(Stat {
        school: String::from(""),
        num_elig: 0,
        num_inelig:0
    })
}
pub fn get_pg_stats(pg_id: i32)->Result<Vec<Stat>>{
    if pg_id == 0 {
        println!("PG 0 COMING...");
        let mut stats = Vec::new();
        
        stats.push(Stat {
            school: String::from("NOT SET"),
            num_elig: 0,
            num_inelig: 0
        });
       return Ok(stats);
    }
    let conn = Connection::open("data.db")?;
    let stats_sql = "SELECT 
                                school,
                                COUNT(CASE WHEN last_status = 'Eligible' THEN child_id ELSE NULL END) AS eligible,
                                COUNT(CASE WHEN last_status = 'Ineligible' THEN child_id ELSE NULL END) AS ineligible
                            FROM afu
                            WHERE pg_id = ?1
                            group by school";
    let mut stmt = conn.prepare(stats_sql)?;
    let mut rows = stmt.query(params![pg_id])?;
    let mut stats = Vec::new();
    while let Some(row) = rows.next()? {
       let stat = Stat {
            school: row.get(0)?,
            num_elig: row.get(1)?,
            num_inelig: row.get(2)?
       };
       stats.push(stat);
    }
    Ok(stats)
}
pub fn get_photographers()->Result<Vec<Photographer>>{
    let conn = Connection::open("data.db")?;
    
    let pg_sql = "SELECT * FROM photographers";
    let mut stmt = conn.prepare(pg_sql)?;
    let mut rows = stmt.query(params![])?;
    let mut photographers: Vec<Photographer>= Vec::new();
    while let Some(row) = rows.next()? {
        let photographer = Photographer{
            id: row.get(0)?,
            name: row.get(1)?
        };
        photographers.push(photographer)
    }

    Ok(photographers)
}
fn get_photographer(school: &str)->Result<Photographer>{
    let sql_pg = "SELECT DISTINCT photographers.* FROM photographers JOIN afu
    ON photographers.id=afu.pg_id
    WHERE school=?1";
    if let Ok(conn) = create_connection(){    
        let mut stmt = conn.prepare(sql_pg)?;
        let mut rows = stmt.query(params![&school])?;
        
        while let Some(row) = rows.next().expect("No rows") {
            let pg = Photographer {
                    id: row.get(0)?,
                    name: row.get(1)?,
            };
            print!("PG from DB, id:{}, name:{}", pg.id, pg.name);
            return Ok(pg);
        }
    }
    Ok(Photographer { id: 0, name: String::from("") })
}
#[tauri::command]
pub fn get_photographer_of(school: &str)->Photographer{
    if let Ok(pg) = get_photographer(school){
        return pg;
    }
    Photographer { id: 0, name: String::from("") }
}

pub struct SchoolIds {
    pub elig_ids : Vec<i32>,
    pub inelig_ids: Vec<i32>
}
impl Serialize for SchoolIds{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {   
        // 1. Serilize the struct
        // 11 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("SchoolIds", 2)?;
        //2. serialize_field
        state.serialize_field("elig_ids", &self.elig_ids)?;
        state.serialize_field("inelig_ids", &self.inelig_ids)?;
        state.end()
    }
}
pub fn get_child_ids(pg_id: i32, school: &str)->Result<SchoolIds>{
    let conn = Connection::open("data.db")?;
    let elig_sql = "select child_id from afu where school = ?1 and pg_id = ?2 and last_status ='Eligible'";
    let inelig_sql = "select child_id from afu where school = ?1 and pg_id = ?2 and last_status ='Ineligible'";

    let mut elig_ids:Vec<i32> = Vec::new();
    let mut stmt = conn.prepare(elig_sql)?;
    let mut rows = stmt.query(params![&school, pg_id])?;
    while let Some(row) = rows.next()? {
        elig_ids.push(row.get(0)?)
    }

    let mut inelig_ids:Vec<i32> = Vec::new();
    let mut stmt_inel = conn.prepare(inelig_sql)?;
    let mut inel_rows = stmt_inel.query(params![&school, pg_id])?;
    while let Some(row) = inel_rows.next()? {
        inelig_ids.push(row.get(0)?)
    }


    Ok(SchoolIds {
        elig_ids,
        inelig_ids
    })
}

pub struct Afu {
    pub child_id: i32,
    pub child_name: String,
    pub sex: String,
    pub last_grade: String,
    pub last_status: String,
    pub school: String,
    pub community: String,
    pub pg_id: i32,
    pub smile_score: i32,
    pub bg_score: i32,
    pub clarity_score: i32
}
impl Serialize for Afu{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {   
        // 1. Serilize the struct
        // 11 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Afu", 11)?;
        //2. serialize_field
        state.serialize_field("child_id", &self.child_id)?;
        state.serialize_field("child_name", &self.child_name)?;
        state.serialize_field("sex", &self.sex)?;
        state.serialize_field("last_grade", &self.last_grade)?;
        state.serialize_field("last_status", &self.last_status)?;
        state.serialize_field("school", &self.school)?;
        state.serialize_field("community", &self.community)?;
        state.serialize_field("pg_id", &self.pg_id)?;
        state.serialize_field("smile_score", &self.smile_score)?;
        state.serialize_field("bg_score", &self.bg_score)?;
        state.serialize_field("clarity_score", &self.clarity_score)?;
        state.end()
    }
}
pub fn get_afu(child_id: i32)-> Result<Afu>{
    let mut afu_row: Afu = Afu{
        child_id:0,
        child_name:String::from(""),
        sex:String::from(""),
        last_grade:String::from(""),
        last_status:String::from(""),
        school:String::from(""),
        community:String::from(""),
        pg_id: 0,
        smile_score:0,
        bg_score:0,
        clarity_score:0
    };
    if let Ok(conn) = create_connection(){
        let afu_sql = "SELECT * FROM afu WHERE child_id = ?1";
        let mut stmt = conn.prepare(afu_sql)?;
        let mut rows = stmt.query(params![child_id])?;
        
        while let Some(row) = rows.next()? {
            afu_row = Afu {
                child_id: row.get(1)?,
                child_name: row.get(2)?,
                sex: row.get(3)?,
                last_grade: row.get(4)?,
                last_status: row.get(5)?,
                school: row.get(6)?,
                community: row.get(7)?,
                pg_id: row.get(8)?,
                smile_score: row.get(9)?,
                bg_score: row.get(10)?,
                clarity_score: row.get(11)?
            }
        }
    }
    Ok(afu_row)
}

pub fn get_schools()->Result<Vec<String>>{
    let mut schools:Vec<String> = Vec::new();
    if let Ok(conn) = create_connection(){
        let schools_sql = "SELECT DISTINCT school FROM afu ORDER BY school";
        let mut stmt = conn.prepare(schools_sql)?;
        let mut rows = stmt.query(params![])?;
        
        while let Some(row) = rows.next()? {
            schools.push(row.get(0)?);
        }
    }
    Ok(schools)
}