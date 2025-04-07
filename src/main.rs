use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Solicitar la ruta del archivo al usuario
    println!("Ingresa la ruta del archivo:");

    let mut entrada: String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la entrada");

    // Eliminar espacios en blanco
    let ruta_archivo: &str = entrada.trim();

    // Usar la ruta predeterminada si el usuario no ingresó nada
    let ruta_archivo: &str = if ruta_archivo.is_empty() {
        "./data/data.csv" // Aqui se define la ruta predeterminada en el directorio del proyecto
    } else {
        ruta_archivo
    };

    // Verificar si la ruta es válida
    if !Path::new(ruta_archivo).exists() {
        eprintln!("Error: El archivo no existe en la ruta: {}", ruta_archivo);
        std::process::exit(1);
    }

    // Abrimos el archivo
    let archivo: File = File::open(ruta_archivo).expect("No se pudo abrir el archivo");
    let lector: io::BufReader<File> = io::BufReader::new(archivo);

    // Variables para el reporte
    let mut balance: f64 = 0.0;
    let mut mayor_monto: f64 = 0.0;
    let mut id_mayor_monto: String = String::new();
    let mut conteo_credito: i32 = 0;
    let mut conteo_debito: i32 = 0;

    // Recorremos el archivo linea por linea
    for (i, linea_resultado) in lector.lines().enumerate() {
        // Intentamos leer la linea. Si falla, se muestra un error y se detiene la ejecucion.
        let linea: String = linea_resultado.expect("No se pudo leer la línea");
        
        // Saltamos la cabecera
        if i == 0 {
            continue;
        }

        // Dividimos la línea por comas para obtener sus columnas
        let columnas: Vec<&str> = linea.split(',').collect();
        if columnas.len() != 3 {
            eprintln!("Línea inválida: {}", linea); 
            continue; // Se salta la línea si no está bien formada
        }

        // Se asigna cada valor a una variable
        let id: &str = columnas[0];
        let tipo: &str = columnas[1];
        let monto: f64 = match columnas[2].parse() { // Convertimos el monto a numero decimal. Si falla, se ignora la línea.
            Ok(valor) => valor,
            Err(_) => {
                eprintln!("Monto inválido en la línea: {}", linea); 
                continue;
            }
        };

        // Procesar según el tipo de transacción: Credito o Debito
        match tipo.trim() {
            "Crédito" => {
                balance += monto; // Se suma al balance
                conteo_credito += 1; // Se suma +1 el conteo de transacciones de credito
            }
            "Débito" => {
                balance -= monto; // Se resta del balance
                conteo_debito += 1; // Se suma +1 el conteo de transacciones de debito
            }
            _ => {
                // Si el tipo no es reconocido, se ignora la línea con una advertencia
                eprintln!("Tipo de transacción desconocido: {}", tipo);
                continue;
            }
        }

        // Se verifica si el monto actual es el mayor hasta ahora
        if monto > mayor_monto {
            mayor_monto = monto; // Se guarda el nuevo monto mayor
            id_mayor_monto = id.to_string(); // Se guarda su id correspondiente
        }
    }

    // Mostramos el reporte
    println!("\nReporte de Transacciones");
    println!("---------------------------------------------");
    println!("Balance Final: {:.2}", balance);
    println!(
        "Transacción de Mayor Monto: ID {} - {:.2}",
        id_mayor_monto, mayor_monto
    );
    println!(
        "Conteo de Transacciones: Crédito: {} Débito: {}",
        conteo_credito, conteo_debito
    );

}
