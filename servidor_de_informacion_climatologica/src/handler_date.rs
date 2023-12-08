use chrono::{NaiveDateTime, Days, Utc};

//Format %Y-%m-%d %H:%M
pub fn parse_str_to_naive(date_str: String)->NaiveDateTime{
    
    let date_time_str = &change_date_format_to_work(date_str.to_string());

    return NaiveDateTime::parse_from_str(&date_time_str,"%d-%m-%Y %H:%M").unwrap();


}


fn change_date_format_to_work(date_str_original:String)-> String{

    return date_str_original;
    
}

pub fn month_name_to_number(month_name:&str)-> i64{
    
    return match month_name{
        "enero" => return 1,
        "febrero" => return 2,
        "marzo" => return 3,
        "abril" => return 4,
        "mayo" => return 5,
        "junio" => return 6,
        "julio" => return 7,
        "agosto" => return 8,
        "septiembre" => return 9,
        "octubre" => return 10,
        "noviembre" => return 11,
        "diciembre" => return 12,
        _ => 0
    }
    
}

pub fn date_since_ago(since_days: u64)-> NaiveDateTime{

    let now= Utc::now().naive_utc();

    let d = Days::new(since_days);
    
    let since_date =now.checked_sub_days(d).unwrap(); 

    return since_date;
}



