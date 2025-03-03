use calamine::{open_workbook_auto, DataType, Reader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 엑셀 파일 경로 설정
    let path = "test.xlsx";
    
    // 파일 열기 (workbook은 mutable이어야 함)
    let mut workbook = open_workbook_auto(path)?;
    
    // 첫 번째 시트 이름을 소유권을 가진 String으로 복제
    let sheet_name = workbook.sheet_names().get(0)
        .ok_or("No sheets found")?
        .to_owned();
    
    // 복제한 시트 이름을 사용하여 시트 읽기
    let range = workbook.worksheet_range(&sheet_name)
        .ok_or("Cannot find sheet")??;
    
    // C열(세 번째 열)의 합계 계산 (첫 행은 헤더로 가정)
    let mut sum = 0.0;
    for row in range.rows().skip(1) {
        if let Some(cell) = row.get(2) {
            match cell {
                DataType::Float(val) => sum += val,
                DataType::Int(val) => sum += *val as f64,
                DataType::String(s) => {
                    if let Ok(val) = s.trim().parse::<f64>() {
                        sum += val;
                    }
                },
                _ => (), // 다른 타입은 무시
            }
        }
    }
    
    println!("C열의 차입수량 합계: {}", sum);
    Ok(())
}
