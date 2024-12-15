use scraping_pexels::resources::scraper::Scraper; // Importa el Scraper correctamente
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Inicializar Scraper con el método asíncrono `new`
    let mut scraper = Scraper::new().await?;

    println!("Introduce un valor de búsqueda:");

    let mut input = String::new(); // Creamos un String vacío para almacenar la entrada del usuario

    std::io::stdin()
        .read_line(&mut input) // Leemos la entrada y la almacenamos en `input`
        .expect("Error al leer la entrada"); // Manejamos posibles errores

    let input = input.trim(); // Eliminamos espacios en blanco o saltos de línea

    println!("Has introducido: {}", input);

    // Llamar al método scrape_all (requiere mutable borrow)

    scraper.scrape_all(input).await?;

    println!("Scraping completado con éxito.");
    Ok(())
}
