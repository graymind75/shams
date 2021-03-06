pub mod consts {
    #![allow(dead_code)]

    pub const CACHE_FOLDER_PATH: &str = "shams";
    pub const CACHE_FILE_NAME: &str = "shams.json";
    pub const TIME_URL: &str = "https://time.ir";
    pub const CALENDAR_SPACE_SIZE: usize = 8;
    pub const MONTH_SPACE_SIZE: usize = 14;
    pub const DAY_SPACE_SIZE: usize = 9;

    pub const SHAMSI_DATE_SELECTOR: &str =
        "span#ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblShamsiNumeral";
    pub const SHAMSI_FULL_DATE_SELECTOR: &str =
        "span#ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblShamsi";

    pub const GEORGIAN_DATE_SELECTOR: &str =
        "span#ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblGregorianNumeral";
    pub const GEORGIAN_FULL_DATE_SELECTOR: &str =
        "span#ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblGregorian";

    pub const CALENDAR_ID: &str = "div#ctl00_cphTop_Sampa_Web_View_EventUI_EventCalendarSimple30cphTop_3732_ecEventCalendar_pnlDayList";
}
