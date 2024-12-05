import type axios from "axios";
import api from "./api";
import type { Anime, Character } from "./models";

export async function getAnimes(): Promise<Anime[]> {
    try {
        const res = (await api.get<Anime[]>("/animes"));
        return res.data;
    } catch (err) {
        console.log(err);
        return [];
    }
}

export async function getCharacters(): Promise<axios.AxiosResponse<Character[], any>> {
    return await api.get<Character[]>("/characters");
}
