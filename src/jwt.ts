const JWT_LOCAL_STORAGE_KEY = "jwt-token";

function store(token: string) {
    localStorage.setItem(JWT_LOCAL_STORAGE_KEY, token);
}

function get() {
    return localStorage.getItem(JWT_LOCAL_STORAGE_KEY);
}

export let JWT_STORE = store;
export let JWT_GET = get;
