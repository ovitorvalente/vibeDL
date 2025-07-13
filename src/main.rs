use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("🎵 Bem-vindo ao vibeDL!");

    loop {
        // --- Entrada do usuário ---
        let url = prompt("🔗 Digite a URL do vídeo ou playlist:");
        if url.trim().is_empty() {
            println!("⚠️ URL não pode estar vazia.");
            continue;
        }

        let audio_only = matches_yes(&prompt("🎧 Deseja baixar apenas o áudio em MP3? (s/n):"));

        let quality = if !audio_only {
            let q = prompt("📺 Qualidade máxima do vídeo? (ex: 720, 1080) ou Enter para melhor:");
            if !q.trim().is_empty() {
                Some(q.trim().to_string())
            } else {
                None
            }
        } else {
            None
        };

        let output_dir = prompt("📂 Caminho onde quer salvar (ou Enter para pasta atual):");
        let output_template = if output_dir.trim().is_empty() {
            "%(title)s.%(ext)s".to_string()
        } else {
            let sep = std::path::MAIN_SEPARATOR;
            let mut path = output_dir.trim().to_string();
            if !path.ends_with(sep) {
                path.push(sep);
            }
            format!("{}%(title)s.%(ext)s", path)
        };

        // --- Montagem do comando yt-dlp ---
        let mut command = Command::new("yt-dlp");

        if audio_only {
            command.args(["-x", "--audio-format", "mp3"]);
        } else if let Some(q) = &quality {
            let format = format!("bestvideo[height<={q}]+bestaudio");
            command.arg("-f").arg(format);
            command.arg("--merge-output-format").arg("mp4");
        } else {
            command.arg("-f").arg("bestvideo+bestaudio");
            command.arg("--merge-output-format").arg("mp4");
        }

        command.arg("-o").arg(&output_template);
        command.arg(url.trim());

        println!("\n🚀 Iniciando download...");
        match command.status() {
            Ok(status) if status.success() => {
                println!("✅ Download concluído com sucesso!\n");
            }
            Ok(status) => {
                println!("❌ Ocorreu um erro (código {}).", status);
            }
            Err(e) => {
                println!("❌ Erro ao executar yt-dlp: {}", e);
            }
        }

        // --- Menu de finalização ---
        let next = prompt("🔁 Digite ENTER para novo download ou 'sair' para fechar:");
        if next.trim().eq_ignore_ascii_case("sair") {
            println!("👋 Encerrando o vibeDL. Até logo!");
            break;
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{msg} ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    resp.trim_end().to_string()
}

fn matches_yes(input: &str) -> bool {
    matches!(input.trim().to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
}
