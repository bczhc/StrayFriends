import {createApp} from "vue";
import naive from 'naive-ui';
import {router} from "./routes.ts";
import AppRoot from "./AppRoot.vue";
import {MessageApiInjection} from "naive-ui/es/message/src/MessageProvider";

createApp(AppRoot)
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

export const CHECK_DIGITS = x => /\d+/.test(x);

export function formatDate(date: Date) {
    return date.toLocaleString();
}

export const messageError = (e: any, message: MessageApiInjection) => {
    message.error(e.toString());
}

export function paginationCount(total: number, pageSize: number) {
    if (total % pageSize === 0) {
        return total / pageSize;
    }
    return Math.floor(total / pageSize) + 1;
}
