import clinte from "axios";
import axios from "axios";

export async function getCharacters() {
  try {
    const characters = await axios.get("/characters");
  } catch (error) {
    console.error(error);
  }
}

// importa o Axios, acho q mudei o nome dele pra client;
//  mas o nome ja ta boninjtnhoo hihihi
// ae faz tipo
//
// characters = axios.get("/characters") e tal; AH, e tem q estar dentro de um try catch
//
//
