// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tauri::{AppHandle, Emitter};

const YABRIDGE_DOCUMENTATION: &str = "https://github.com/robbert-vdh/yabridge#usage";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstallPlan {
    Fedora,
    Arch,
}

#[derive(Serialize)]
struct DependencyStatus {
    distribution: String,
    wine_installed: bool,
    yabridge_installed: bool,
    automatic_install_supported: bool,
    installation_plan: String,
    documentation_url: &'static str,
}

// Declara um comando que o Tauri exporta para o Vue
#[tauri::command]
fn executar_yabridge(comando: String) -> Result<String, String> {
    match comando.as_str() {
        "status" | "sync" => executar_yabridge_argumentos(&[&comando]),
        _ => Err("Ação yabridgectl não permitida.".to_string()),
    }
}

#[tauri::command]
fn gerenciar_locais_plugins(acao: String, caminho: Option<String>) -> Result<String, String> {
    let argumentos = argumentos_gerenciamento(&acao, caminho.as_deref())?;
    let argumentos = argumentos.iter().map(String::as_str).collect::<Vec<_>>();
    executar_yabridge_argumentos(&argumentos)
}

fn argumentos_gerenciamento(acao: &str, caminho: Option<&str>) -> Result<Vec<String>, String> {
    match (acao, caminho) {
        ("add", Some(caminho)) if !caminho.trim().is_empty() && Path::new(caminho).is_dir() => {
            Ok(vec!["add".to_string(), caminho.to_string()])
        }
        ("add", _) => Err("Selecione um diretório de plugins existente.".to_string()),
        ("rm", Some(caminho)) if !caminho.trim().is_empty() => {
            Ok(vec!["rm".to_string(), caminho.to_string()])
        }
        ("rm", _) => Err("Informe o diretório que deseja remover da lista.".to_string()),
        ("list" | "status", None) => Ok(vec![acao.to_string()]),
        _ => Err("Ação ou argumentos inválidos para gerenciar locais de plugins.".to_string()),
    }
}

fn executar_yabridge_argumentos(argumentos: &[&str]) -> Result<String, String> {
    let executavel = localizar_yabridgectl()?;
    let output = std::process::Command::new(&executavel)
        .args(argumentos)
        .output()
        .map_err(|erro| format!("Falha ao iniciar {}: {}", executavel.display(), erro))?;

    let stdout = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string();
    let stderr = String::from_utf8_lossy(&output.stderr)
        .trim_end()
        .to_string();

    if output.status.success() {
        match (stdout.is_empty(), stderr.is_empty()) {
            (false, false) => Ok(format!("{}\n\nSTDERR:\n{}", stdout, stderr)),
            (false, true) => Ok(stdout),
            (true, false) => Ok(format!("STDERR:\n{}", stderr)),
            (true, true) => Ok("Comando concluído sem saída.".to_string()),
        }
    } else {
        let detalhes = match (stdout.is_empty(), stderr.is_empty()) {
            (false, false) => format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr),
            (false, true) => format!("STDOUT:\n{}", stdout),
            (true, false) => format!("STDERR:\n{}", stderr),
            (true, true) => "O processo não retornou detalhes.".to_string(),
        };
        Err(format!(
            "yabridgectl terminou com {}.\n{}",
            output.status, detalhes
        ))
    }
}

fn localizar_yabridgectl() -> Result<std::path::PathBuf, String> {
    let mut candidatos = Vec::new();

    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        candidatos.push(home.join(".local/bin/yabridgectl"));
        candidatos.push(home.join(".local/share/yabridge/yabridgectl"));
    }

    candidatos.push(std::path::PathBuf::from("/usr/local/bin/yabridgectl"));
    candidatos.push(std::path::PathBuf::from("/usr/bin/yabridgectl"));

    if let Some(path) = std::env::var_os("PATH") {
        candidatos.extend(std::env::split_paths(&path).map(|pasta| pasta.join("yabridgectl")));
    }

    candidatos
        .into_iter()
        .find_map(|candidato| {
            std::fs::canonicalize(&candidato)
                .ok()
                .filter(|caminho| caminho.is_file())
        })
        .ok_or_else(|| {
            "yabridgectl não encontrado. Verifique ~/.local/bin, ~/.local/share/yabridge, /usr/local/bin, /usr/bin ou PATH.".to_string()
        })
}

#[tauri::command]
fn detectar_dependencias() -> DependencyStatus {
    let release = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
    let id = os_release_value(&release, "ID").to_lowercase();
    let version = os_release_value(&release, "VERSION_ID");
    let distribution = os_release_value(&release, "PRETTY_NAME");
    let distribution = if distribution.is_empty() {
        "Distribuição Linux desconhecida".to_string()
    } else {
        distribution
    };
    let atomic = Path::new("/run/ostree-booted").exists();
    let plan = selecionar_plano(&id, &version, std::env::consts::ARCH, atomic);

    let installation_plan = match plan {
        Some(InstallPlan::Fedora) => "Fedora: habilitar o COPR comunitário patrickl/wine-tkg, recomendado pelo Yabridge, e instalar Wine Staging 9.21 (64/32 bits), Yabridge e componentes requeridos.".to_string(),
        Some(InstallPlan::Arch) => "Arch/Manjaro: instalar wine-staging, yabridge e yabridgectl dos repositórios configurados via pacman.".to_string(),
        None if atomic => "Instalação automática desabilitada em sistemas Fedora Atomic. Use o fluxo rpm-ostree documentado pela sua distribuição.".to_string(),
        None if id == "fedora" => "Instalação automática disponível apenas em Fedora 43/44 x86_64 não Atomic, conforme o COPR documentado.".to_string(),
        None if id == "debian" || id == "ubuntu" || id == "linuxmint" || id == "pop" => "A documentação recomenda Wine Staging do repositório WineHQ nestas distribuições; não será instalado um pacote potencialmente incompatível dos repositórios padrão.".to_string(),
        None => "Não há uma rota automática verificada para esta distribuição. Consulte a documentação oficial para instalar Wine Staging e Yabridge.".to_string(),
    };

    DependencyStatus {
        distribution,
        wine_installed: ["wine", "wine64", "wine-staging"]
            .iter()
            .any(|programa| executavel_disponivel(programa)),
        yabridge_installed: localizar_yabridgectl().is_ok(),
        automatic_install_supported: plan.is_some(),
        installation_plan,
        documentation_url: YABRIDGE_DOCUMENTATION,
    }
}

fn os_release_value(contents: &str, key: &str) -> String {
    contents
        .lines()
        .find_map(|line| {
            let (candidate, value) = line.split_once('=')?;
            (candidate == key).then(|| value.trim().trim_matches('"').replace("\\\"", "\""))
        })
        .unwrap_or_default()
}

fn selecionar_plano(id: &str, version: &str, arch: &str, atomic: bool) -> Option<InstallPlan> {
    if atomic || arch != "x86_64" {
        return None;
    }

    match id {
        "fedora" if matches!(version, "43" | "44") => Some(InstallPlan::Fedora),
        "arch" | "manjaro" => Some(InstallPlan::Arch),
        _ => None,
    }
}

fn executavel_disponivel(programa: &str) -> bool {
    let mut diretorios = vec![PathBuf::from("/usr/bin"), PathBuf::from("/usr/local/bin")];
    if let Some(path) = std::env::var_os("PATH") {
        diretorios.extend(std::env::split_paths(&path));
    }

    diretorios
        .into_iter()
        .map(|diretorio| diretorio.join(programa))
        .any(|caminho| caminho.is_file())
}

#[tauri::command]
async fn instalar_dependencias(app: AppHandle) -> Result<String, String> {
    let release = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
    let id = os_release_value(&release, "ID").to_lowercase();
    let version = os_release_value(&release, "VERSION_ID");
    let atomic = Path::new("/run/ostree-booted").exists();
    let plan = selecionar_plano(&id, &version, std::env::consts::ARCH, atomic)
        .ok_or_else(|| "Instalação automática não suportada para este sistema. Consulte a documentação oficial do Yabridge.".to_string())?;

    tauri::async_runtime::spawn_blocking(move || match plan {
        InstallPlan::Fedora => instalar_fedora(&version, &app),
        InstallPlan::Arch => instalar_arch(&app),
    })
    .await
    .map_err(|erro| format!("A tarefa de instalação foi interrompida: {}", erro))?
}

fn instalar_fedora(version: &str, app: &AppHandle) -> Result<String, String> {
    let wine_mono = if version == "43" {
        "wine-mono-bin"
    } else {
        "wine-mono"
    };
    let mut instalar_args = vec!["-y", "install", "--refresh", "wine.x86_64", "wine.i686"];
    instalar_args.push(wine_mono);
    instalar_args.extend([
        "mingw32-wine-gecko",
        "mingw64-wine-gecko",
        "wine-dxvk",
        "winetricks",
        "yabridge",
        "libcurl-gnutls",
    ]);
    let habilitar_args = ["-y", "copr", "enable", "patrickl/wine-tkg"];
    let passos: [(&str, &[&str]); 2] = [
        ("Habilitar o repositório Wine/Yabridge", &habilitar_args),
        ("Instalar Wine e Yabridge", &instalar_args),
    ];

    executar_passos_privilegiados("/usr/bin/dnf", &passos, app)
}

fn instalar_arch(app: &AppHandle) -> Result<String, String> {
    let passos: [(&str, &[&str]); 1] = [(
        "Instalar Wine Staging e Yabridge",
        &[
            "-S",
            "--needed",
            "--noconfirm",
            "wine-staging",
            "yabridge",
            "yabridgectl",
        ],
    )];

    executar_passos_privilegiados("/usr/bin/pacman", &passos, app)
}

fn executar_passos_privilegiados(
    programa: &str,
    passos: &[(&str, &[&str])],
    app: &AppHandle,
) -> Result<String, String> {
    if !Path::new("/usr/bin/pkexec").is_file() {
        return Err(
            "pkexec não está instalado; não é possível abrir a autorização gráfica do sistema."
                .to_string(),
        );
    }

    let mut saida = Vec::new();
    for (descricao, argumentos) in passos {
        let mensagem = format!("Aguardando autorização do sistema: {}.", descricao);
        let _ = app.emit("installation-progress", mensagem);
        let mut processo = Command::new("/usr/bin/pkexec")
            .arg(programa)
            .args(*argumentos)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|erro| format!("{}: falha ao iniciar o gerenciador: {}", descricao, erro))?;
        let stdout = processo
            .stdout
            .take()
            .ok_or_else(|| format!("{}: não foi possível ler stdout.", descricao))?;
        let stderr = processo
            .stderr
            .take()
            .ok_or_else(|| format!("{}: não foi possível ler stderr.", descricao))?;
        let (sender, receiver) = std::sync::mpsc::channel::<(&'static str, String)>();
        ler_saida("stdout", stdout, sender.clone());
        ler_saida("stderr", stderr, sender.clone());
        drop(sender);

        let mut detalhes = Vec::new();
        let status = loop {
            while let Ok((stream_name, line)) = receiver.try_recv() {
                emitir_linha(app, stream_name, line, &mut detalhes);
            }

            if let Some(status) = processo.try_wait().map_err(|erro| {
                format!("{}: falha aguardando o gerenciador: {}", descricao, erro)
            })? {
                break status;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        };

        for (stream_name, line) in receiver {
            emitir_linha(app, stream_name, line, &mut detalhes);
        }

        if !status.success() {
            let detalhes = detalhes.join("\n");
            return Err(format!("{} falhou ({}). {}", descricao, status, detalhes));
        }

        saida.push(format!("{} concluído.", descricao));
    }

    Ok(saida.join("\n"))
}

fn emitir_linha(app: &AppHandle, stream_name: &str, line: String, detalhes: &mut Vec<String>) {
    let linha_saida = if stream_name == "stderr" {
        format!("stderr: {}", line)
    } else {
        line
    };
    let _ = app.emit("installation-progress", linha_saida.clone());
    detalhes.push(linha_saida);
}

fn ler_saida<R>(
    nome_stream: &'static str,
    stream: R,
    sender: std::sync::mpsc::Sender<(&'static str, String)>,
) where
    R: Read + Send + 'static,
{
    std::thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            match line {
                Ok(line) => {
                    if sender.send((nome_stream, line)).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{InstallPlan, argumentos_gerenciamento, os_release_value, selecionar_plano};

    #[test]
    fn recognizes_supported_fedora_versions() {
        assert_eq!(
            selecionar_plano("fedora", "44", "x86_64", false),
            Some(InstallPlan::Fedora)
        );
        assert_eq!(selecionar_plano("fedora", "45", "x86_64", false), None);
    }

    #[test]
    fn blocks_atomic_and_unsupported_architectures() {
        assert_eq!(selecionar_plano("fedora", "44", "x86_64", true), None);
        assert_eq!(selecionar_plano("fedora", "44", "aarch64", false), None);
    }

    #[test]
    fn detects_arch_based_distributions() {
        assert_eq!(
            selecionar_plano("manjaro", "rolling", "x86_64", false),
            Some(InstallPlan::Arch)
        );
    }

    #[test]
    fn parses_os_release_values() {
        assert_eq!(
            os_release_value("ID=fedora\nPRETTY_NAME=\"Fedora Linux 44\"", "PRETTY_NAME"),
            "Fedora Linux 44"
        );
    }

    #[test]
    fn builds_expected_plugin_location_arguments() {
        let diretorio =
            std::env::temp_dir().join(format!("vst-gui-test-{}-VST Plugins", std::process::id()));
        std::fs::create_dir_all(&diretorio).unwrap();
        let caminho = diretorio.to_string_lossy().into_owned();

        assert_eq!(
            argumentos_gerenciamento("add", Some(&caminho)).unwrap(),
            vec!["add", caminho.as_str()]
        );
        assert_eq!(
            argumentos_gerenciamento("rm", Some(&caminho)).unwrap(),
            vec!["rm", caminho.as_str()]
        );
        assert_eq!(
            argumentos_gerenciamento("list", None).unwrap(),
            vec!["list"]
        );
        assert_eq!(
            argumentos_gerenciamento("status", None).unwrap(),
            vec!["status"]
        );

        std::fs::remove_dir(diretorio).unwrap();
    }

    #[test]
    fn rejects_invalid_plugin_location_actions() {
        assert!(argumentos_gerenciamento("add", Some("/path/that/does/not/exist")).is_err());
        assert!(argumentos_gerenciamento("rm", None).is_err());
        assert!(argumentos_gerenciamento("sync", None).is_err());
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            executar_yabridge,
            gerenciar_locais_plugins,
            detectar_dependencias,
            instalar_dependencias
        ])
        .run(tauri::generate_context!()) // <--- Adicionado o '!' aqui
        .expect("erro ao executar a aplicação tauri");
}
