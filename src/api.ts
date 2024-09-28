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

export async function apiRequest(url: string, form: object = {}) {
    await delay(SIMULATE_API_DELAY);
    console.log(url, form);
    let axios = useAxios();
    let result: AxiosResponse<any> = await axios.post(url, form, {
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            'Authorization': `Bearer ${JWT_GET() || ''}`,
        }
    });
    console.log(result);
    return ApiResponse.from(result.data);
}
