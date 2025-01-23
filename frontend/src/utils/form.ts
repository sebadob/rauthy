import {CalendarDate, getDayOfWeek, isWeekend, startOfMonth, startOfWeek} from "@internationalized/date";

/**
 * Returns a Unix timestamp from the given DateTime input
 */
export function unixTsFromDateInput(inputDate: string) {
    let d = Date.parse(inputDate);
    if (isNaN(d)) {
        return;
    }
    return d / 1000;
}

export const DAY_MILLIS = 1000 * 60 * 60 * 24;

export interface Day {
    day: number,
    month: number,
    year: number,
    isWeekend: boolean,
}

export function dayToString(day: Day) {
    return `${day.year}-${day.month > 9 ? day.month : '0' + day.month}-${day.day > 9 ? day.day : '0' + day.day}`;
}

export interface Week {
    days: Day[],
}

/*
Returns all weeks in the month for the given date.
Appends days from the month before and the one after if there is an overlap.
 */
export function getWeeksInMonth(date: CalendarDate, locale: string): Week[] {
    const thisMonth = date.month;

    date = startOfMonth(date);
    date = startOfWeek(date, locale);
    let weeks: Week[] = [];

    const pushWeekDay = (days: Day[]) => {
        days.push({
            day: date.day,
            month: date.month,
            year: date.year,
            isWeekend: isWeekend(date, locale),
        });
        date = date.add({days: 1});
    }

    const pushWeek = () => {
        let week: Week = {
            // number: getWeekOfYear(date),
            days: [],
        };
        for (let i = 0; i < 7; i++) {
            pushWeekDay(week.days);
        }
        weeks.push(week);
    }

    // build the first week with possible overlap into month before
    pushWeek();

    // build weeks afterward until we get into the next month
    while (date.month === thisMonth) {
        pushWeek();
    }

    // fill up the last week with possible overlap into next month
    let lastWeek = weeks[weeks.length - 1];
    let dayOfWeek = getDayOfWeek(date, locale);
    while (dayOfWeek > 0) {
        pushWeekDay(lastWeek.days);
        dayOfWeek = getDayOfWeek(date, locale);
    }
    weeks[weeks.length - 1] = lastWeek;

    return weeks;
}

/**
 * Converts the DateTime string inside the given URLSearchParams to a Unix timestamp
 */
export function formParamsDateToTs(key: string, params: URLSearchParams) {
    let ts = unixTsFromDateInput(params.get(key) || '');
    params.set(key, ts?.toString() || '');
    return params;
}

/**
 * Returns `now` in Date only correct format for input element
 * Date will be `local` and probably needs to be combined with the `<TZ>` component.
 */
export function fmtDateInput(date?: Date) {
    const d = date || new Date();

    let dd: string | number = d.getDate();
    // let dd: string | number = d.getUTCDate();
    if (dd < 10) {
        dd = '0' + dd;
    }
    let mm: string | number = d.getMonth() + 1;
    // let mm: string | number = d.getUTCMonth() + 1;
    if (mm < 10) {
        mm = '0' + mm;
    }
    const yyyy = d.getUTCFullYear();

    return `${yyyy}-${mm}-${dd}`;
}

/**
 * Returns `now` in Time only correct format for input element.
 * Time will be `local` and probably needs to be combined with the `<TZ>` component.
 */
export function fmtTimeInput(date?: Date) {
    const d = date || new Date();

    let hr: string | number = d.getHours();
    // let hr: string | number = d.getUTCHours();
    if (hr < 10) {
        hr = '0' + hr;
    }
    let mn: string | number = d.getMinutes();
    // let mn: string | number = d.getUTCMinutes();
    if (mn < 10) {
        mn = '0' + mn;
    }

    return `${hr}:${mn}`;
}

/**
 * Converts the given `params` into an object that makes the TS compiler happy.
 * CAUTION: It does NOT check the integrity! Uses unchecked typecasting!
 */
export function paramsInto<T>(params: URLSearchParams): T {
    let res: any = {};
    for (let p of params) {
        res[p[0]] = p[1] || undefined;
    }
    return res as T;
}
