use crate::interfaces::videos;
use reqwest;
use std::borrow::Cow;
use std::error::Error;
use std::fs::{self};
use std::io::Write;
use thirtyfour::prelude::*;

#[derive(Debug)]
pub struct Scraper {
    pub thirtyfour_instance: WebDriver,
    pub videos: Vec<videos::Video>,
    pub reqwester: reqwest::Client,
}

impl Scraper {
    // Método para inicializar el Scraper
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let capabilities = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", capabilities).await?;
        let reqwester = reqwest::Client::new();

        Ok(Self {
            thirtyfour_instance: driver,
            videos: Vec::new(),
            reqwester,
        })
    }

    /// Hacer scraping de todos los videos
    pub async fn scrape_all(&mut self, search_param: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("https://www.pexels.com/search/videos/{}", search_param);

        self.thirtyfour_instance.get(url).await?;

        // Aquí irían las llamadas a otras funciones de scraping
        self.videos = self.scrape_videos().await?;

        // Tomamos los videos en una variable temporal
        let videos_to_download = self.videos.clone();

        let mut index = 0;

        // Ahora iteramos sobre la variable temporal
        for video in videos_to_download {
            index = index + 1;
            let file_name = format!("{:?}.mp4", index);
            println!("filenameeee {:?}", file_name);
            self.download_video(video.url.as_ref(), &file_name).await?; // Usa as_ref() para convertir de Cow a &str
        }

        println!("{:?}", self.videos);
        Ok(())
    }

    /// Hacer scraping de un video
    pub async fn scrape_videos(&mut self) -> Result<Vec<videos::Video>, Box<dyn Error>> {
        // Ejemplo de scraping (esto debería ajustarse a tu HTML específico)
        let col1 = self
            .thirtyfour_instance
            .find_all(By::Css(
                "div.BreakpointGrid_column__9MIoh:nth-child(1) > div",
            ))
            .await?;

        let col2 = self
            .thirtyfour_instance
            .find_all(By::Css(
                "div.BreakpointGrid_column__9MIoh:nth-child(2) > div",
            ))
            .await?;

        let col3 = self
            .thirtyfour_instance
            .find_all(By::Css(
                "div.BreakpointGrid_column__9MIoh:nth-child(3) > div",
            ))
            .await?;

        // Combina todos los elementos de col1, col2 y col3 en un solo Vec<WebElement>
        let mut elements: Vec<WebElement> = Vec::new();
        elements.extend(col1);
        elements.extend(col2);
        elements.extend(col3);

        println!("elements: {:?}", elements);

        let mut scraped_videos = Vec::new();

        for element in elements {
            // Hacer clic en el elemento para abrirlo
            element.click().await?;
            element.wait_until();

            // Obtener la URL (href) del elemento
            let url =
                if let Ok(url_element) = element.find(By::Css("div > article > a > video")).await {
                    // Extraer el atributo href
                    url_element
                        .attr("src") // Obtener el atributo href
                        .await?
                } else {
                    Some(String::from("Sin URL")) // Fallback si no se encuentra el elemento con la clase .url
                };

            // Almacenar el video extraído
            scraped_videos.push(videos::Video {
                title: Cow::Borrowed("title"),
                url: Cow::Owned(url.unwrap()),
            });
        }

        Ok(scraped_videos)
    }

    // Descargar video
    async fn download_video(&mut self, url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
        println!("Iniciando descarga del video: {:?}", url);

        if url == "Sin URL" {
            println!("URL inválida, omitiendo...");
            return Ok(());
        }

        // Iniciar la solicitud GET
        let mut response = self.reqwester.get(url).send().await?;

        // Verificar que la respuesta sea exitosa
        if !response.status().is_success() {
            eprintln!("Error al descargar el video: {:?}", response.status());
            return Err(format!("Error al obtener el video desde la URL: {:?}", url).into());
        }

        // Crear directorio si no existe
        let output_dir = "../files";
        fs::create_dir_all(output_dir)?;

        // Construir la ruta completa del archivo
        let file_path = format!("{}/{}", output_dir, file_name);

        println!("filepath: {:?}", file_path);
        // Abrir archivo para escribir
        let mut file = fs::File::create_new(&file_path)?;

        // Descargar en chunks
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
            println!("chunk: {:?}", chunk);
        }

        println!("Video guardado exitosamente en: {}", file_path);
        Ok(())
    }
}
