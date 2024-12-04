import axios from "axios";

import dotenv from "dotenv";
dotenv.config();

const api = axios.create({
    baseURL: process.env.DATABASE_URL,
    headers: {
        "Content-Type": "application/json",
    },
});

export default api;