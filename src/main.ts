import {createApp} from "vue";
import App from "./App.vue";
import naive from 'naive-ui';
import {router} from "./routes.ts";

createApp(App)
    .use(router)
    .use(naive)
    .mount('#app');

export function checkEmail(s: string) {
    // noinspection RegExpRedundantEscape,RegExpUnnecessaryNonCapturingGroup
    let EMAIL_REGEX = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;
    return EMAIL_REGEX.test(s);
}

export function delay(ms: number) {
    return new Promise(e => {
        setTimeout(() => {
            e();
        }, ms);
    });
}

export const WWW_FORM_URLENCODED_HEADER = {'content-type': 'application/x-www-form-urlencoded'};

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
