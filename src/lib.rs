use std::{error::Error, fs::File};
use std::fs::create_dir_all;
use std::io::{copy, Write}; // Добавили Write для write_all
use std::path::Path;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub mod cli;
pub use crate::cli::Cli;

// Функция для получения пути к файлу относительно корня
fn get_relative_path(resource_url: &Url, base_url: &Url) -> String {
    let base_path = base_url.path();
    let resource_path = resource_url.path();
    
    // Убираем базовый путь сайта, чтобы сохранить структуру относительных путей
    if resource_path.starts_with(base_path) {
        return resource_path[base_path.len()..].to_string();
    }
    resource_path.to_string()
}

async fn download_file(url: &str, save_path: &str) -> Result<(), Box<dyn Error>> {
    // Создаем директории, если они не существуют
    let save_path_path = Path::new(save_path);
    if let Some(parent) = save_path_path.parent() {
        create_dir_all(parent)?; // Создаем все необходимые директории
    }

    let response = reqwest::get(url).await?;
    let mut file = File::create(save_path)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;

    println!("Скачан файл: {}", save_path);
    Ok(())
}

// Функция для скачивания ресурсов (например, изображений, CSS, JS и т.д.)
async fn download_resources(
    document: &Html,
    selector: &Selector,
    attribute: &str,
    base_url: &Url,
    root_folder: &str,
) -> Result<(), Box<dyn Error>> {
    for element in document.select(selector) {
        if let Some(resource_url) = element.value().attr(attribute) {
            // Преобразуем относительный путь ресурса в абсолютный
            let resource_url = base_url.join(resource_url)?;

            // Получаем относительный путь ресурса
            let relative_path = get_relative_path(&resource_url, base_url);

            // Формируем полный путь для сохранения ресурса
            let save_path = format!("{}/{}", root_folder, relative_path);

            // Скачиваем и сохраняем файл
            download_file(resource_url.as_str(), &save_path).await?;
        }
    }

    Ok(())
}

// Основная функция для обработки страницы и скачивания всех ресурсов
#[tokio::main]
pub async fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let url = &cli.url; // Укажите URL для скачивания
    let root_folder = "downloaded_site"; // Корневая папка для сохранения сайтаx

    create_dir_all(root_folder)?;

    // Загружаем страницу
    let response = Client::new().get(url).send().await?;
    let body = response.text().await?;

    // Парсим HTML
    let document = Html::parse_document(&body);

    // Базовый URL для обработки относительных путей
    let base_url = Url::parse(url)?;

    // Сохраняем основной HTML-файл
    let html_file_path = format!("{}/index.html", root_folder);
    let mut file = File::create(&html_file_path)?;
    file.write_all(body.as_bytes())?;  // Теперь метод write_all доступен

    // Скачиваем ресурсы
    let img_selector = Selector::parse("img").unwrap(); // Ищем все теги <img>
    let css_selector = Selector::parse("link[rel='stylesheet']").unwrap(); // Ищем теги CSS
    let js_selector = Selector::parse("script[src]").unwrap(); // Ищем теги JS

    // Скачиваем все изображения, стили и скрипты
    download_resources(&document, &img_selector, "src", &base_url, root_folder).await?;
    download_resources(&document, &css_selector, "href", &base_url, root_folder).await?;
    download_resources(&document, &js_selector, "src", &base_url, root_folder).await?;

    Ok(())
}
