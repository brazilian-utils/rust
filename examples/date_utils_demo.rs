use brazilian_utils::date_utils::{convert_date_to_text, is_holiday};
use chrono::NaiveDate;

fn main() {
    println!("=== Demonstração do Módulo Date Utils ===\n");
    
    // Conversão de datas para texto
    println!("1. Conversão de datas para texto:");
    println!("   01/01/2024: {:?}", convert_date_to_text("01/01/2024").unwrap());
    println!("   15/08/1990: {:?}", convert_date_to_text("15/08/1990").unwrap());
    println!("   25/12/2000: {:?}", convert_date_to_text("25/12/2000").unwrap());
    println!("   07/09/2024: {:?}\n", convert_date_to_text("07/09/2024").unwrap());
    
    // Verificação de feriados nacionais
    println!("2. Verificação de feriados nacionais:");
    
    let new_year = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    println!("   01/01/2024 (Ano Novo): {}", 
        if is_holiday(new_year, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let tiradentes = NaiveDate::from_ymd_opt(2024, 4, 21).unwrap();
    println!("   21/04/2024 (Tiradentes): {}", 
        if is_holiday(tiradentes, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let labor_day = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
    println!("   01/05/2024 (Dia do Trabalhador): {}", 
        if is_holiday(labor_day, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let good_friday = NaiveDate::from_ymd_opt(2024, 3, 29).unwrap();
    println!("   29/03/2024 (Sexta-feira Santa): {}", 
        if is_holiday(good_friday, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let independence = NaiveDate::from_ymd_opt(2024, 9, 7).unwrap();
    println!("   07/09/2024 (Independência): {}", 
        if is_holiday(independence, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let christmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    println!("   25/12/2024 (Natal): {}", 
        if is_holiday(christmas, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let regular_day = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
    println!("   02/01/2024 (Dia comum): {}\n", 
        if is_holiday(regular_day, None).unwrap() { "FERIADO" } else { "Dia comum" });
    
    // Verificação de feriados estaduais
    println!("3. Verificação de feriados estaduais:");
    
    let bahia_independence = NaiveDate::from_ymd_opt(2024, 7, 2).unwrap();
    println!("   02/07/2024 (Independência da Bahia):");
    println!("     - Na Bahia (BA): {}", 
        if is_holiday(bahia_independence, Some("BA")).unwrap() { "FERIADO" } else { "Dia comum" });
    println!("     - Em São Paulo (SP): {}", 
        if is_holiday(bahia_independence, Some("SP")).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let sp_revolution = NaiveDate::from_ymd_opt(2024, 7, 9).unwrap();
    println!("   09/07/2024 (Revolução Constitucionalista):");
    println!("     - Em São Paulo (SP): {}", 
        if is_holiday(sp_revolution, Some("SP")).unwrap() { "FERIADO" } else { "Dia comum" });
    println!("     - No Rio de Janeiro (RJ): {}", 
        if is_holiday(sp_revolution, Some("RJ")).unwrap() { "FERIADO" } else { "Dia comum" });
    
    let black_awareness = NaiveDate::from_ymd_opt(2024, 11, 20).unwrap();
    println!("   20/11/2024 (Consciência Negra):");
    println!("     - Em Alagoas (AL): {}", 
        if is_holiday(black_awareness, Some("AL")).unwrap() { "FERIADO" } else { "Dia comum" });
    println!("     - Em São Paulo (SP): {}", 
        if is_holiday(black_awareness, Some("SP")).unwrap() { "FERIADO" } else { "Dia comum" });
}
