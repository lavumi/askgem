use tauri::{WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn open_gemini() {
    // 아무것도 안해도 됨, 메인 윈도우가 이미 Gemini를 로드함
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 메인 윈도우를 Gemini 사이트로 설정
            let _window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://gemini.google.com/app".parse().unwrap()),
            )
            .title("AskGem - Gemini AI")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .center()
            .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_gemini])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
