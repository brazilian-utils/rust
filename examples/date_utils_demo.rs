use brazilian_utils::date_utils::{convert_date_to_text, is_holiday, IsHolidayParams};
use chrono::NaiveDate;

fn main() {
    println!("=== Demonstração do Módulo Date Utils ===\n");

    // Conversão de datas para texto
    println!("1. Conversão de datas para texto:");
    println!(
        "   01/01/2024: {:?}",
        convert_date_to_text("01/01/2024").unwrap()
    );
    println!(
        "   15/08/1990: {:?}",
        convert_date_to_text("15/08/1990").unwrap()
    );
    println!(
        "   25/12/2000: {:?}",
        convert_date_to_text("25/12/2000").unwrap()
    );
    println!(
        "   07/09/2024: {:?}\n",
        convert_date_to_text("07/09/2024").unwrap()
    );

    // Verificação de feriados nacionais
    println!("2. Verificação de feriados nacionais:");

    let new_year = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    println!(
        "   01/01/2024 (Ano Novo): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(new_year), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let tiradentes = NaiveDate::from_ymd_opt(2024, 4, 21).unwrap();
    println!(
        "   21/04/2024 (Tiradentes): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(tiradentes), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let labor_day = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
    println!(
        "   01/05/2024 (Dia do Trabalhador): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(labor_day), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let good_friday = NaiveDate::from_ymd_opt(2024, 3, 29).unwrap();
    println!(
        "   29/03/2024 (Sexta-feira Santa): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(good_friday), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let independence = NaiveDate::from_ymd_opt(2024, 9, 7).unwrap();
    println!(
        "   07/09/2024 (Independência): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(independence), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let christmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    println!(
        "   25/12/2024 (Natal): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(christmas), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let regular_day = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
    println!(
        "   02/01/2024 (Dia comum): {}\n",
        if is_holiday(Some(IsHolidayParams { date: Some(regular_day), uf: None })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    // Verificação de feriados estaduais
    println!("3. Verificação de feriados estaduais:");

    let bahia_independence = NaiveDate::from_ymd_opt(2024, 7, 2).unwrap();
    println!("   02/07/2024 (Independência da Bahia):");
    println!(
        "     - Na Bahia (BA): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(bahia_independence), uf: Some("BA".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );
    println!(
        "     - Em São Paulo (SP): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(bahia_independence), uf: Some("SP".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let sp_revolution = NaiveDate::from_ymd_opt(2024, 7, 9).unwrap();
    println!("   09/07/2024 (Revolução Constitucionalista):");
    println!(
        "     - Em São Paulo (SP): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(sp_revolution), uf: Some("SP".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );
    println!(
        "     - No Rio de Janeiro (RJ): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(sp_revolution), uf: Some("RJ".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );

    let black_awareness = NaiveDate::from_ymd_opt(2024, 11, 20).unwrap();
    println!("   20/11/2024 (Consciência Negra):");
    println!(
        "     - Em Alagoas (AL): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(black_awareness), uf: Some("AL".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );
    println!(
        "     - Em São Paulo (SP): {}",
        if is_holiday(Some(IsHolidayParams { date: Some(black_awareness), uf: Some("SP".to_string()) })).unwrap() {
            "FERIADO"
        } else {
            "Dia comum"
        }
    );
}
