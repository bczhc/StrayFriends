import axios, {AxiosResponse} from "axios";
import {JWT_GET} from "./jwt.ts";
import {delay} from "./main.ts";

const SIMULATE_API_DELAY = 1000 /* 1s */;

export class ApiResponse {
    code: number;
    data?: object;
    message?: string;

    success() {
        return this.code == 0;
    }

    messageOrEmpty() {
        return this.message || '';
    }

    dataAs<T>() {
        return this.data as (T | null);
    }

    static from(obj: any) {
        let r = new ApiResponse();
        r.data = obj['data'];
        r.code = obj['code'];
        r.message = obj['message'];
        return r;
    }
}

export function useAxios() {
    return axios;
}

export const authHeaders = () => {
    return {
        'Authorization': `Bearer ${JWT_GET() || ''}`,
    };
}

const authedUrlencodedHeader = () => {
    return {
        'Content-Type': 'application/x-www-form-urlencoded',
        ...authHeaders()
    };
}

export async function apiPost(url: string, form: object = {}) {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    console.log(url, form);

    let result: AxiosResponse<any> = await axios.post(url, form, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    return ApiResponse.from(result.data);
}

export async function apiGet(url: string) {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    let result: AxiosResponse<any> = await axios.get(url, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    return ApiResponse.from(result.data);
}

export async function apiPut(url: string) {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    let result: AxiosResponse<any> = await axios.put(url, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    return ApiResponse.from(result.data);
}
