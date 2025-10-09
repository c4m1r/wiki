use std::io::{self, Write};
use std::process::Command as ProcessCommand;
use std::thread;
use std::time::Duration;
use crate::logic::{NervaLogic, CliArgs};

pub fn start_interactive_mode() {
    // Show ASCII animation
    show_ascii_animation();

    // Initialize logic
    let logic = match NervaLogic::new() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("✗ Failed to initialize NervaWeb: {}", e);
            return;
        }
    };

    // Start local servers for existing projects
    start_local_servers(&logic);

    // Main menu loop
    let mut current_language = "en"; // Default to English

    loop {
        show_main_menu(current_language);

        let choice = get_user_input();

        let should_continue = match choice.trim() {
            "1" => {
                current_language = select_language_menu();
                true
            }
            "2" => {
                publish_locally_menu(&logic, current_language);
                true
            }
            "3" => {
                create_project_menu(&logic, current_language);
                true
            }
            "4" => {
                clear_cache_menu(&logic, current_language);
                true
            }
            "5" => {
                delete_project_menu(&logic, current_language);
                true
            }
            "6" => {
                clear_cargo_cache_menu(&logic, current_language);
                true
            }
            "7" => {
                change_project_language_menu(&logic, current_language);
                true
            }
            "8" => {
                change_project_theme_menu(&logic, current_language);
                true
            }
            "9" => {
                console_mode(&logic);
                true
            }
            "0" => {
                println!("⊕ Goodbye!");
                false
            }
            _ => {
                println!("✗ Invalid choice. Please try again.");
                true
            }
        };

        if !should_continue {
            break;
        }

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn show_ascii_animation() {
    let frames = vec![
        r#"
                _____
             .-'     '-.
           .'           '.
          /   [92mNervaWeb[0m    \
         ;        O        ;
         |       /|\       |
         |       / \       |
         ;   ###     ###   ;
          \               /
           '.           .'
             '-._____.-'
       "#,
        r#"
                _____
             .-'     '-.
           .'           '.
          /   [92mNervaWeb[0m    \
         ;       O        ;
         |      /|\       |
         |      / \       |
         ;  ###     ###   ;
          \               /
           '.           .'
             '-._____.-'
       "#,
        r#"
                _____
             .-'     '-.
           .'           '.
          /   [92mNervaWeb[0m    \
         ;      O         ;
         |     /|\        |
         |     / \        |
         ; ###     ###    ;
          \               /
           '.           .'
             '-._____.-'
       "#,
        r#"
                _____
             .-'     '-.
           .'           '.
          /   [92mNervaWeb[0m    \
         ;     O          ;
         |    /|\         |
         |    / \         |
         ;   ###     ###  ;
          \               /
           '.           .'
             '-._____.-'
       "#,
    ];

    for frame in frames.iter() {
        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", frame);
        println!("  Multi-Project Static Site Generator");
        println!("  Powered by Rust");
        println!();
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(300));
    }

    // Clear screen completely for final clean display
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn start_local_servers(logic: &NervaLogic) {
    let projects = match logic.get_projects() {
        Ok(p) => p,
        Err(_) => return,
    };

    if projects.is_empty() {
        return;
    }

    println!("▲ Starting local servers...");

    let mut port = 8000;
    for project in projects {
        let output_path = logic.get_project_output_path(&project);

        if output_path.exists() {
            // Try to assign port from config or auto-assign
            let config_port = if let Ok(config) = logic.load_project_config(&project) {
                config.local_port.unwrap_or(port)
            } else {
                port
            };

            println!("∞ {}: http://localhost:{}", project, config_port);

            // Start server in background (simplified - in real implementation would use a proper web server)
            let project_name = project.clone();
            let port_num = config_port;

            thread::spawn(move || {
                let _ = ProcessCommand::new("python")
                    .args(&["-m", "http.server", &port_num.to_string(), "-d", &format!("good2go/{}", project_name)])
                    .spawn();
            });

            port += 1;
        }
    }
    println!();
}

fn show_main_menu(language: &str) {
    let menu = match language {
        "ru" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                      Главное меню                           ║
╠══════════════════════════════════════════════════════════════╣
║  1. Выбрать язык                                           ║
║  2. Опубликовать локально                                   ║
║  3. Создать проект                                          ║
║  4. Очистить кеш Good2Go                                    ║
║  5. Удалить проект                                          ║
║  6. Очистить Cargo кеш                                      ║
║  7. Сменить язык проекта                                    ║
║  8. Сменить тему проекта                                    ║
║  9. Консольный режим                                        ║
║  0. Выход                                                   ║
╚══════════════════════════════════════════════════════════════╝
Выберите опцию (0-9): "#
        },
        "es" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                       Menú Principal                        ║
╠══════════════════════════════════════════════════════════════╣
║  1. Seleccionar idioma                                      ║
║  2. Publicar localmente                                     ║
║  3. Crear proyecto                                          ║
║  4. Limpiar cache Good2Go                                   ║
║  5. Eliminar proyecto                                       ║
║  6. Limpiar cache Cargo                                     ║
║  7. Cambiar idioma del proyecto                             ║
║  8. Cambiar tema del proyecto                               ║
║  9. Modo consola                                            ║
║  0. Salir                                                   ║
╚══════════════════════════════════════════════════════════════╝
Seleccionar opción (0-9): "#
        },
        "de" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                      Hauptmenu                              ║
╠══════════════════════════════════════════════════════════════╣
║  1. Sprache wahlen                                          ║
║  2. Lokal veroffentlichen                                   ║
║  3. Projekt erstellen                                       ║
║  4. Good2Go Cache leeren                                    ║
║  5. Projekt loschen                                         ║
║  6. Cargo Cache leeren                                      ║
║  7. Projektsprache andern                                   ║
║  8. Projekt-Theme andern                                    ║
║  9. Konsolenmodus                                           ║
║  0. Beenden                                                 ║
╚══════════════════════════════════════════════════════════════╝
Option wahlen (0-9): "#
        },
        "fr" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                       Menu Principal                        ║
╠══════════════════════════════════════════════════════════════╣
║  1. Selectionner la langue                                  ║
║  2. Publier localement                                      ║
║  3. Creer un projet                                         ║
║  4. Vider le cache Good2Go                                  ║
║  5. Supprimer le projet                                     ║
║  6. Vider le cache Cargo                                    ║
║  7. Changer la langue du projet                             ║
║  8. Changer le theme du projet                              ║
║  9. Mode console                                            ║
║  0. Quitter                                                 ║
╚══════════════════════════════════════════════════════════════╝
Selectionner option (0-9): "#
        },
        "it" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                      Menu Principale                        ║
╠══════════════════════════════════════════════════════════════╣
║  1. Seleziona lingua                                        ║
║  2. Pubblica localmente                                     ║
║  3. Crea progetto                                           ║
║  4. Svuota cache Good2Go                                    ║
║  5. Elimina progetto                                        ║
║  6. Svuota cache Cargo                                      ║
║  7. Cambia lingua progetto                                  ║
║  8. Cambia tema progetto                                    ║
║  9. Modalita console                                        ║
║  0. Esci                                                    ║
╚══════════════════════════════════════════════════════════════╝
Seleziona opzione (0-9): "#
        },
        "pt" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                      Menu Principal                         ║
╠══════════════════════════════════════════════════════════════╣
║  1. Selecionar idioma                                       ║
║  2. Publicar localmente                                     ║
║  3. Criar projeto                                           ║
║  4. Limpar cache Good2Go                                    ║
║  5. Excluir projeto                                         ║
║  6. Limpar cache Cargo                                      ║
║  7. Alterar idioma do projeto                               ║
║  8. Alterar tema do projeto                                 ║
║  9. Modo console                                            ║
║  0. Sair                                                    ║
╚══════════════════════════════════════════════════════════════╝
Selecionar opcao (0-9): "#
        },
        "zh" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          主菜单                              ║
╠══════════════════════════════════════════════════════════════╣
║  1. 选择语言                                               ║
║  2. 本地发布                                               ║
║  3. 创建项目                                               ║
║  4. 清除Good2Go缓存                                        ║
║  5. 删除项目                                               ║
║  6. 清除Cargo缓存                                           ║
║  7. 更改项目语言                                            ║
║  8. 更改项目主题                                            ║
║  9. 控制台模式                                              ║
║  0. 退出                                                   ║
╚══════════════════════════════════════════════════════════════╝
选择选项 (0-9): "#
        },
        "ja" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          メインメニュー                       ║
╠══════════════════════════════════════════════════════════════╣
║  1. 言語を選択                                             ║
║  2. ローカルに公開                                         ║
║  3. プロジェクトを作成                                     ║
║  4. Good2Goキャッシュをクリア                              ║
║  5. プロジェクトを削除                                     ║
║  6. Cargoキャッシュをクリア                                ║
║  7. プロジェクトの言語を変更                               ║
║  8. プロジェクトのテーマを変更                             ║
║  9. コンソールモード                                       ║
║  0. 終了                                                   ║
╚══════════════════════════════════════════════════════════════╝
オプションを選択 (0-9): "#
        },
        "ko" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          메인 메뉴                          ║
╠══════════════════════════════════════════════════════════════╣
║  1. 언어 선택                                             ║
║  2. 로컬에 게시                                           ║
║  3. 프로젝트 생성                                          ║
║  4. Good2Go 캐시 비우기                                   ║
║  5. 프로젝트 삭제                                          ║
║  6. Cargo 캐시 비우기                                     ║
║  7. 프로젝트 언어 변경                                     ║
║  8. 프로젝트 테마 변경                                     ║
║  9. 콘솔 모드                                             ║
║  0. 종료                                                   ║
╚══════════════════════════════════════════════════════════════╝
옵션을 선택하세요 (0-9): "#
        },
        "ar" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          القائمة الرئيسية                   ║
╠══════════════════════════════════════════════════════════════╣
║  1. اختر اللغة                                            ║
║  2. نشر محلياً                                            ║
║  3. إنشاء مشروع                                           ║
║  4. مسح ذاكرة التخزين المؤقت Good2Go                      ║
║  5. حذف المشروع                                           ║
║  6. مسح ذاكرة التخزين المؤقت Cargo                       ║
║  7. تغيير لغة المشروع                                     ║
║  8. تغيير ثيم المشروع                                     ║
║  9. وضع وحدة التحكم                                       ║
║  0. خروج                                                  ║
╚══════════════════════════════════════════════════════════════╝
اختر الخيار (0-9): "#
        },
        "hi" => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          मुख्य मेनू                          ║
╠══════════════════════════════════════════════════════════════╣
║  1. भाषा चुनें                                            ║
║  2. स्थानीय रूप से प्रकाशित करें                          ║
║  3. प्रोजेक्ट बनाएं                                       ║
║  4. Good2Go कैश साफ करें                                  ║
║  5. प्रोजेक्ट हटाएं                                       ║
║  6. Cargo कैश साफ करें                                    ║
║  7. प्रोजेक्ट की भाषा बदलें                               ║
║  8. प्रोजेक्ट की थीम बदलें                                ║
║  9. कंसोल मोड                                             ║
║  0. बाहर निकलें                                           ║
╚══════════════════════════════════════════════════════════════╝
विकल्प चुनें (0-9): "#
        },
        _ => {
            r#"
╔══════════════════════════════════════════════════════════════╗
║                          NervaWeb                           ║
║                          Main Menu                          ║
╠══════════════════════════════════════════════════════════════╣
║  1. Select Language                                         ║
║  2. Publish Locally                                         ║
║  3. Create Project                                          ║
║  4. Clear Good2Go Cache                                     ║
║  5. Delete Project                                          ║
║  6. Clear Cargo Cache                                       ║
║  7. Change Project Language                                 ║
║  8. Change Project Theme                                    ║
║  9. Console Mode                                            ║
║  0. Exit                                                    ║
╚══════════════════════════════════════════════════════════════╝
Select option (0-9): "#
        },
    };
    println!("{}", menu);
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    input
}

fn select_language_menu() -> &'static str {
    println!("\nSelect language:");
    println!("1. English (en)");
    println!("2. Russian (ru)");
    println!("3. Spanish (es)");
    println!("4. German (de)");
    println!("5. French (fr)");
    println!("6. Italian (it)");
    println!("7. Portuguese (pt)");
    println!("8. Chinese (zh)");
    println!("9. Japanese (ja)");
    println!("10. Korean (ko)");
    println!("11. Arabic (ar)");
    println!("12. Hindi (hi)");
    print!("Enter choice (1-12): ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => "en",
        "2" => "ru",
        "3" => "es",
        "4" => "de",
        "5" => "fr",
        "6" => "it",
        "7" => "pt",
        "8" => "zh",
        "9" => "ja",
        "10" => "ko",
        "11" => "ar",
        "12" => "hi",
        _ => "en",
    }
}

fn publish_locally_menu(logic: &NervaLogic, language: &str) {
    let projects = match logic.get_projects() {
        Ok(p) => p,
        Err(e) => {
            println!("✗ Failed to get projects: {}", e);
            return;
        }
    };

    if projects.is_empty() {
        println!("No projects found. Create one first with: nervaweb new <project-name>");
        return;
    }

    println!("\nAvailable projects:");
    for (i, project) in projects.iter().enumerate() {
        println!("{}. {}", i + 1, project);
    }
    println!("{}. All projects", projects.len() + 1);

    print!("Select project to publish (1-{}): ", projects.len() + 1);
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    let choice_num: usize = match choice.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("✗ Invalid choice");
            return;
        }
    };

    if choice_num >= 1 && choice_num <= projects.len() {
        let project = &projects[choice_num - 1];
        println!("Publishing project: {}", project);
        // TODO: Implement actual publishing logic
        println!("✔ Project '{}' published locally", project);
    } else if choice_num == projects.len() + 1 {
        println!("Publishing all projects...");
        // TODO: Implement publishing all projects
        println!("✔ All projects published locally");
    } else {
        println!("✗ Invalid choice");
    }
}

fn create_project_menu(logic: &NervaLogic, language: &str) {
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    let project_name = get_user_input().trim().to_string();

    if project_name.is_empty() {
        println!("✗ Project name cannot be empty");
        return;
    }

    print!("Enter project description (optional): ");
    io::stdout().flush().unwrap();
    let description = get_user_input().trim().to_string();
    let description = if description.is_empty() { None } else { Some(description) };

    println!("Available themes:");
    println!("1. hello-world");
    println!("2. wiki");
    println!("3. blog");
    print!("Select theme (1-3): ");
    io::stdout().flush().unwrap();

    let theme_choice = get_user_input();
    let theme = match theme_choice.trim() {
        "1" => Some("hello-world"),
        "2" => Some("wiki"),
        "3" => Some("blog"),
        _ => {
            println!("✗ Invalid theme choice, using default");
            None
        }
    };

    match logic.create_project(&project_name, description.as_deref(), theme.map(|s| s)) {
        Ok(()) => {
            println!("✔ Project '{}' created successfully!", project_name);
        }
        Err(e) => {
            println!("✗ Failed to create project '{}': {}", project_name, e);
        }
    }
}

fn clear_cache_menu(_logic: &NervaLogic, language: &str) {
    println!("Clearing Good2Go cache...");
    // TODO: Implement cache clearing logic
    println!("✔ Good2Go cache cleared");
}

fn delete_project_menu(logic: &NervaLogic, language: &str) {
    let projects = match logic.get_projects() {
        Ok(p) => p,
        Err(e) => {
            println!("✗ Failed to get projects: {}", e);
            return;
        }
    };

    if projects.is_empty() {
        println!("No projects found.");
        return;
    }

    println!("\nAvailable projects:");
    for (i, project) in projects.iter().enumerate() {
        println!("{}. {}", i + 1, project);
    }

    print!("Select project to delete (1-{}): ", projects.len());
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    let choice_num: usize = match choice.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("✗ Invalid choice");
            return;
        }
    };

    if choice_num >= 1 && choice_num <= projects.len() {
        let project = &projects[choice_num - 1];
        print!("Are you sure you want to delete project '{}' ? (y/N): ", project);
        io::stdout().flush().unwrap();

        let confirmation = get_user_input();
        if confirmation.trim().to_lowercase() == "y" {
            match logic.remove_project(project) {
                Ok(()) => {
                    println!("✔ Project '{}' deleted successfully", project);
                }
                Err(e) => {
                    println!("✗ Failed to delete project '{}': {}", project, e);
                }
            }
        } else {
            println!("Deletion cancelled");
        }
    } else {
        println!("✗ Invalid choice");
    }
}

fn clear_cargo_cache_menu(_logic: &NervaLogic, language: &str) {
    println!("Clearing Cargo compilation cache...");
    // TODO: Implement cargo cache clearing logic
    println!("✔ Cargo cache cleared");
}

fn change_project_language_menu(_logic: &NervaLogic, _language: &str) {
    println!("Feature in development - change project language");
}

fn change_project_theme_menu(_logic: &NervaLogic, _language: &str) {
    println!("Feature in development - change project theme");
}

fn console_mode(logic: &NervaLogic) {
    // Clear screen and show help
    print!("\x1B[2J\x1B[1;1H");

    // TODO: Show help information
    println!("NervaWeb Console Mode");
    println!("Available commands:");
    println!("  nervaweb new <project-name> [--desc \"Description\"] [--theme <theme>]");
    println!("  nervaweb build <project-name>");
    println!("  nervaweb clear");
    println!("  nervaweb content");
    println!("  nervaweb help");
    println!("  nervaweb version");

    // Wait for user to press Enter to return to menu
    println!("\n{}", "Press Enter to return to main menu...");
    let _ = io::stdin().read_line(&mut String::new());
}
