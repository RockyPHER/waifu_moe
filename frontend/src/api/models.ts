export interface Anime {
    id: number;
    name: string;
    origin: string;
    logoUrl: string;
    updatedAt: string;
    createdAt: string;
}

export interface Character {
    id: number;
    name: string;
    bio: string;
    age: number;
    birthDay: string;
    height: number;
    weight: number;
    gender: string;
    pannelUrl: string;
    cardUrl: string;
    numLikes: number;
    numDislikes: number;
    updatedAt: string;
    createdAt: string;
}