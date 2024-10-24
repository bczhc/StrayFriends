import axios, {AxiosResponse} from "axios";
import {JWT_GET} from "./jwt.ts";
import {delay} from "./main.ts";

const SIMULATE_API_DELAY = 500 /* 500ms */;

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

async function resolveApiResponse(res: ApiResponse): Promise<object | null> {
    return new Promise((resolve, reject) => {
        if (res.success()) {
            resolve(res.data);
        } else {
            reject(res.messageOrEmpty());
        }
    });
}

export async function apiPost(url: string, form: object = {}): Promise<object | null> {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    console.log(url, form);

    let result: AxiosResponse<any> = await axios.post(url, form, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    let res = ApiResponse.from(result.data);
    return resolveApiResponse(res);
}

export async function apiGet(url: string) {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    let result: AxiosResponse<any> = await axios.get(url, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    return resolveApiResponse(ApiResponse.from(result.data));
}

export async function apiPut(url: string, form: object = {}) {
    await delay(SIMULATE_API_DELAY);
    let axios = useAxios();
    let result: AxiosResponse<any> = await axios.put(url, form, {
        headers: authedUrlencodedHeader(),
    });
    console.log(result);
    return resolveApiResponse(ApiResponse.from(result.data));
}

export function parseNUploadOnFinishEvent(event: Event) {
    let resText = (event.target as XMLHttpRequest).response;
    let res = JSON.parse(resText);
    return {
        digest: res['data']
    };
}

export function imageUrl(digest: string) {
    return `/api/image/${digest}`;
}

export interface AnimalCardInfo {
    username: string,
    userAvatarImageId: string,
    name: string,
    description: string,
    content: string,
    creationTime: number,
    imageIdList: string[],
    postId: number,
    adopted: boolean,
}

export interface AdoptionRequest {
    requestId: number,
    postUid: number,
    animalPostId: number,
    requestDetails: string,
    mobileNumber: string,
}
