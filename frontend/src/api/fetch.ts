import type axios from "axios";
import api from "./api";
import type { Anime, Character } from "./models";

export async function getAnimes(): Promise<axios.AxiosResponse<Anime[], any>> {
    return await api.get<Anime[]>("/animes");
}

export async function getCharacters(): Promise<axios.AxiosResponse<Character[], any>> {
    return await api.get<Character[]>("/characters");
}

export async function getCharactersByAnime(id: number): Promise<axios.AxiosResponse<Character[], any>> {
    return await api.get<Character[]>(`/animes/${id}/characters`);
}