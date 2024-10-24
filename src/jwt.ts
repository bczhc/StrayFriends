import * as base64 from "base64-js";

const JWT_LOCAL_STORAGE_KEY = "jwt-token";

function store(token: string) {
    localStorage.setItem(JWT_LOCAL_STORAGE_KEY, token);
}

function get() {
    return localStorage.getItem(JWT_LOCAL_STORAGE_KEY);
}

export let JWT_STORE = store;
export let JWT_GET = get;
export let JWT_CLEAR = () => {
    localStorage.removeItem(JWT_LOCAL_STORAGE_KEY);
};

export let JWT_GET_CLAIMS = () => {
    let token = JWT_GET();
    if (!token) return null;

    let claimsBase64 = token.split('.')[1];
    let claims = new TextDecoder().decode(base64.toByteArray(claimsBase64));
    try {
        return JSON.parse(claims) as Claims;
    } catch (e: any) {
        return null;
    }
};

interface Claims {
    user: User,
}

interface User {
    id: number,
    name: string,
    email: string,
    avatarImageId?: string,
    admin: boolean,
}

export function checkAdmin() {
    let admin = JWT_GET_CLAIMS()?.user.admin;
    return admin === undefined ? false : admin;
}
