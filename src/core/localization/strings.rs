use std::collections::HashMap;
use crate::core::models::AppLanguage;

pub struct Strings {
    en_map: HashMap<&'static str, &'static str>,
    fa_map: HashMap<&'static str, &'static str>,
}

impl Strings {
    pub fn new() -> Self {
        let mut en_map = HashMap::new();
        let mut fa_map = HashMap::new();

        // General & Titles
        en_map.insert("AppTitle", "Awayra");
        fa_map.insert("AppTitle", "Awayra");

        en_map.insert("Starting", "Starting...");
        fa_map.insert("Starting", "در حال راه‌اندازی...");

        // Statuses
        en_map.insert("StatusRunning", "Reminders active");
        fa_map.insert("StatusRunning", "یادآورها فعال هستند");

        en_map.insert("StatusPaused", "Paused");
        fa_map.insert("StatusPaused", "متوقف شده");

        en_map.insert("StatusPausedIdle", "Paused while idle");
        fa_map.insert("StatusPausedIdle", "متوقف شده هنگام عدم فعالیت");

        en_map.insert("StatusIdle", "Idle");
        fa_map.insert("StatusIdle", "در حال استراحت");

        en_map.insert("StatusConfigurationPaused", "Settings open");
        fa_map.insert("StatusConfigurationPaused", "تنظیمات باز است");

        en_map.insert("StatusOutsideWorkHours", "Outside work hours");
        fa_map.insert("StatusOutsideWorkHours", "خارج از ساعات کاری");

        en_map.insert("StatusBreakActive", "Break in progress");
        fa_map.insert("StatusBreakActive", "استراحت در جریان است");

        en_map.insert("StatusSnoozed", "Snoozed");
        fa_map.insert("StatusSnoozed", "به تعویق افتاد");

        en_map.insert("StatusDisabled", "Reminders disabled");
        fa_map.insert("StatusDisabled", "یادآورها غیرفعال هستند");

        // Break Names & Cards
        en_map.insert("EyeReset", "Eye Reset");
        fa_map.insert("EyeReset", "استراحت چشم");

        en_map.insert("MoveBreak", "Move Break");
        fa_map.insert("MoveBreak", "استراحت حرکتی");

        en_map.insert("CardEyeReset", "EYE RESET");
        fa_map.insert("CardEyeReset", "استراحت چشم");

        en_map.insert("CardMoveBreak", "MOVE BREAK");
        fa_map.insert("CardMoveBreak", "استراحت حرکتی");

        en_map.insert("Enabled", "Enabled");
        fa_map.insert("Enabled", "فعال");

        en_map.insert("Disabled", "Disabled");
        fa_map.insert("Disabled", "غیرفعال");

        // Action Buttons
        en_map.insert("Pause", "Pause");
        fa_map.insert("Pause", "توقف");

        en_map.insert("Resume", "Resume");
        fa_map.insert("Resume", "ادامه");

        en_map.insert("EyeResetNow", "Eye Reset Now");
        fa_map.insert("EyeResetNow", "استراحت چشم الان");

        en_map.insert("MoveBreakNow", "Move Break Now");
        fa_map.insert("MoveBreakNow", "استراحت حرکتی الان");

        en_map.insert("Settings", "Settings");
        fa_map.insert("Settings", "تنظیمات");

        en_map.insert("Quit", "Quit");
        fa_map.insert("Quit", "خروج");

        en_map.insert("OpenAwayra", "Open Awayra");
        fa_map.insert("OpenAwayra", "باز کردن Awayra");

        en_map.insert("Mute", "Mute");
        fa_map.insert("Mute", "بی‌صدا");

        en_map.insert("Muted", "Muted");
        fa_map.insert("Muted", "بی‌صدا شد");

        en_map.insert("Skip", "Skip");
        fa_map.insert("Skip", "رد کردن");

        en_map.insert("Snooze", "Snooze");
        fa_map.insert("Snooze", "تعویق");

        en_map.insert("Complete", "Complete");
        fa_map.insert("Complete", "تکمیل");

        // Dashboard Stats
        en_map.insert("TodayEyeCompleted", "Eye Resets");
        fa_map.insert("TodayEyeCompleted", "استراحت چشم");

        en_map.insert("TodayMoveCompleted", "Move Breaks");
        fa_map.insert("TodayMoveCompleted", "استراحت حرکتی");

        en_map.insert("TodaySkipped", "Skipped");
        fa_map.insert("TodaySkipped", "رد شده");

        en_map.insert("TodaySnoozed", "Snoozed");
        fa_map.insert("TodaySnoozed", "به تعویق افتاده");

        en_map.insert("AboutSupportTitle", "About & Support Awayra");
        fa_map.insert("AboutSupportTitle", "درباره و حمایت از Awayra");

        en_map.insert("AboutSupportSub", "Built with love for people who spend long hours at a computer.");
        fa_map.insert("AboutSupportSub", "ساخته شده با عشق برای افرادی که ساعات طولانی پشت کامپیوتر هستند.");

        // Overlay Instructions
        en_map.insert("EyeResetInstructionDistance", "Look at a distant object for a few breaths.");
        fa_map.insert("EyeResetInstructionDistance", "برای چند ثانیه به یک جسم در فاصله دور نگاه کنید.");

        en_map.insert("EyeResetInstructionBlink", "Blink naturally and relax your face.");
        fa_map.insert("EyeResetInstructionBlink", "به آرامی پلک بزنید و عضلات صورت را شل کنید.");

        en_map.insert("TakeABreak", "Take a break and rest your eyes");
        fa_map.insert("TakeABreak", "استراحت کنید و به چشمانتان آرامش دهید");

        en_map.insert("SecondsRemaining", "seconds remaining");
        fa_map.insert("SecondsRemaining", "ثانیه باقی‌مانده");

        // Move Activities
        en_map.insert("MoveActivityStand", "Stand up and reset your posture.");
        fa_map.insert("MoveActivityStand", "بایستید و وضعیت بدن خود را تنظیم کنید.");

        en_map.insert("MoveActivityWalk", "Walk briefly away from your desk.");
        fa_map.insert("MoveActivityWalk", "کمی از میز کار خود فاصله گرفته و قدم بزنید.");

        en_map.insert("MoveActivityShoulders", "Relax your shoulders and unclench your jaw.");
        fa_map.insert("MoveActivityShoulders", "شانه‌های خود را رها کنید و فک خود را شل کنید.");

        en_map.insert("MoveActivityNeck", "Gently move your neck within a comfortable range.");
        fa_map.insert("MoveActivityNeck", "گردن خود را به آرامی در حد راحت بچرخانید.");

        en_map.insert("MoveActivityStretch", "Stretch your arms gently without forcing.");
        fa_map.insert("MoveActivityStretch", "دست‌های خود را به آرامی و بدون فشار بکشید.");

        // Exercise Step Titles & Details (Eye)
        en_map.insert("EyeStep1Title", "Distance Focus");
        fa_map.insert("EyeStep1Title", "تمرکز بر فاصله");

        en_map.insert("EyeStep1Desc", "Look at a distant object 20 feet away to relax your eye focusing muscles.");
        fa_map.insert("EyeStep1Desc", "به یک جسم در فاصله دور نگاه کنید تا عضلات چشم شل شوند.");

        en_map.insert("EyeStep2Title", "Blink Consciously");
        fa_map.insert("EyeStep2Title", "پلک زدن آگاهانه");

        en_map.insert("EyeStep2Desc", "Blink slowly 5-10 times to moisten and soothe your eyes.");
        fa_map.insert("EyeStep2Desc", "آرام ۵ تا ۱۰ بار پلک بزنید تا چشمانتان مرطوب شوند.");

        en_map.insert("EyeStep3Title", "Side to Side");
        fa_map.insert("EyeStep3Title", "حرکت به طرفین");

        en_map.insert("EyeStep3Desc", "Slowly roll your eyes left to right 3 times.");
        fa_map.insert("EyeStep3Desc", "به آرامی چشمان خود را ۳ بار به چپ و راست بچرخانید.");

        en_map.insert("EyeStep4Title", "Palming");
        fa_map.insert("EyeStep4Title", "گرما بخشی");

        en_map.insert("EyeStep4Desc", "Rub hands together to generate warmth and gently cup them over your closed eyes.");
        fa_map.insert("EyeStep4Desc", "دستان خود را به هم بمالید تا گرم شوند و روی چشمان بسته بگذارید.");

        // Exercise Step Titles & Details (Move)
        en_map.insert("MoveStep1Title", "Full Body Stretch");
        fa_map.insert("MoveStep1Title", "کشش کامل بدن");

        en_map.insert("MoveStep1Desc", "Stand up straight, reach both arms up toward the ceiling, and stretch your legs.");
        fa_map.insert("MoveStep1Desc", "بایستید، دستان خود را به سمت بالا بکشید و بدن را کشش دهید.");

        en_map.insert("MoveStep2Title", "Neck & Shoulder Rolls");
        fa_map.insert("MoveStep2Title", "چرخش گردن و شانه");

        en_map.insert("MoveStep2Desc", "Roll your shoulders backward 5 times, then slowly tilt your neck side to side.");
        fa_map.insert("MoveStep2Desc", "شانه‌ها را ۵ بار به عقب بچرخانید و گردن را به طرفین متمایل کنید.");

        en_map.insert("MoveStep3Title", "Hydration Break");
        fa_map.insert("MoveStep3Title", "نوشیدن آب");

        en_map.insert("MoveStep3Desc", "Walk to get a fresh glass of water and take deep breaths.");
        fa_map.insert("MoveStep3Desc", "کمی قدم بزنید، یک لیوان آب بنوشید و نفس عمق بکشید.");

        en_map.insert("MoveStep4Title", "Torso Twists");
        fa_map.insert("MoveStep4Title", "چرخش بالاتنه");

        en_map.insert("MoveStep4Desc", "Stand up, place hands on hips, and gently twist your upper body left and right.");
        fa_map.insert("MoveStep4Desc", "بایستید، دست‌ها را روی کمر بگذارید و بالاتنه را به چپ و راست بچرخانید.");

        en_map.insert("StepProgress", "Step {0} of {1}: {2}");
        fa_map.insert("StepProgress", "گام {0} از {1}: {2}");

        en_map.insert("ActivityLabel", "Activity");
        fa_map.insert("ActivityLabel", "فعالیت");

        // Tray Strings
        en_map.insert("TrayTooltipNextBreak", "Next break in {0}");
        fa_map.insert("TrayTooltipNextBreak", "استراحت بعدی در {0}");

        en_map.insert("TrayPauseReminders", "Pause Reminders");
        fa_map.insert("TrayPauseReminders", "توقف یادآورها");

        en_map.insert("TrayResumeReminders", "Resume Reminders");
        fa_map.insert("TrayResumeReminders", "ادامه یادآورها");

        en_map.insert("TrayPauseFor", "Pause for…");
        fa_map.insert("TrayPauseFor", "توقف برای…");

        en_map.insert("TrayPause30m", "Pause for 30 minutes");
        fa_map.insert("TrayPause30m", "توقف به مدت ۳۰ دقیقه");

        en_map.insert("TrayPause1h", "Pause for 1 hour");
        fa_map.insert("TrayPause1h", "توقف به مدت ۱ ساعت");

        en_map.insert("TrayPauseTomorrow", "Pause until tomorrow");
        fa_map.insert("TrayPauseTomorrow", "توقف تا فردا");

        en_map.insert("TrayRemindersPaused", "Reminders paused");
        fa_map.insert("TrayRemindersPaused", "یادآورها متوقف شدند");

        en_map.insert("TrayBreakActive", "break active");
        fa_map.insert("TrayBreakActive", "استراحت فعال است");

        // Settings Sections & Labels
        en_map.insert("SettingsTitle", "Awayra Settings");
        fa_map.insert("SettingsTitle", "تنظیمات Awayra");

        en_map.insert("SettingsSubtitle", "Configure break intervals, sounds, appearance, idle detection, and system options.");
        fa_map.insert("SettingsSubtitle", "تنظیم فواصل استراحت، صداها، ظاهر، تشخیص عدم فعالیت و گزینه‌های سیستم.");

        en_map.insert("SettingsEyeResetSection", "Eye Reset");
        fa_map.insert("SettingsEyeResetSection", "استراحت چشم");

        en_map.insert("SettingsEnableEyeReset", "Enable Eye Reset");
        fa_map.insert("SettingsEnableEyeReset", "فعال‌سازی استراحت چشم");

        en_map.insert("SettingsMoveBreakSection", "Move Break");
        fa_map.insert("SettingsMoveBreakSection", "استراحت حرکتی");

        en_map.insert("SettingsEnableMoveBreak", "Enable Move Break");
        fa_map.insert("SettingsEnableMoveBreak", "فعال‌سازی استراحت حرکتی");

        en_map.insert("SettingsInterval", "Interval");
        fa_map.insert("SettingsInterval", "فاصله زمانی");

        en_map.insert("SettingsDuration", "Duration");
        fa_map.insert("SettingsDuration", "مدت زمان");

        en_map.insert("SettingsPlaySoundOnStart", "Play sound on start");
        fa_map.insert("SettingsPlaySoundOnStart", "پخش صدا در شروع استراحت");

        en_map.insert("SettingsSoundSection", "Sound Theme & Audio");
        fa_map.insert("SettingsSoundSection", "تم صوتی و صدا");

        en_map.insert("SettingsSoundTheme", "Sound Theme");
        fa_map.insert("SettingsSoundTheme", "تم صوتی");

        en_map.insert("SettingsVolume", "Volume");
        fa_map.insert("SettingsVolume", "حجم صدا");

        en_map.insert("SettingsRepeatInterval", "Repeat interval (0=off)");
        fa_map.insert("SettingsRepeatInterval", "فاصله تکرار (۰=غیرفعال)");

        en_map.insert("SettingsPreviewSound", "Preview Sound");
        fa_map.insert("SettingsPreviewSound", "پیش‌نمایش صدا");

        en_map.insert("SettingsTestSound", "Test sound");
        fa_map.insert("SettingsTestSound", "تست صدا");

        en_map.insert("SettingsBehaviorSection", "Reminder Behavior");
        fa_map.insert("SettingsBehaviorSection", "رفتار یادآورها");

        en_map.insert("SettingsAllowSkip", "Allow skip");
        fa_map.insert("SettingsAllowSkip", "اجازه رد کردن");

        en_map.insert("SettingsAllowSnooze", "Allow snooze");
        fa_map.insert("SettingsAllowSnooze", "اجازه تعویق");

        en_map.insert("SettingsSnoozeDuration", "Snooze duration");
        fa_map.insert("SettingsSnoozeDuration", "مدت زمان تعویق");

        en_map.insert("SettingsIdleSection", "Idle & Work Hours");
        fa_map.insert("SettingsIdleSection", "عدم فعالیت و ساعات کاری");

        en_map.insert("SettingsPauseWhileIdle", "Pause reminders when idle");
        fa_map.insert("SettingsPauseWhileIdle", "توقف یادآورها هنگام عدم فعالیت");

        en_map.insert("SettingsIdleThreshold", "Idle threshold");
        fa_map.insert("SettingsIdleThreshold", "آستانه عدم فعالیت");

        en_map.insert("SettingsWorkHoursEnabled", "Enable work hours filter");
        fa_map.insert("SettingsWorkHoursEnabled", "فعال‌سازی فیلتر ساعات کاری");

        en_map.insert("SettingsWorkStart", "Work start time");
        fa_map.insert("SettingsWorkStart", "زمان شروع کار");

        en_map.insert("SettingsWorkEnd", "Work end time");
        fa_map.insert("SettingsWorkEnd", "زمان پایان کار");

        en_map.insert("SettingsAppearanceSection", "Appearance & Language");
        fa_map.insert("SettingsAppearanceSection", "ظاهر و زبان");

        en_map.insert("SettingsLanguage", "Language");
        fa_map.insert("SettingsLanguage", "زبان برنامه");

        en_map.insert("SettingsGlassClarity", "Glass Clarity");
        fa_map.insert("SettingsGlassClarity", "شفافیت شیشه");

        en_map.insert("SettingsReducedMotion", "Reduced motion (disable animations)");
        fa_map.insert("SettingsReducedMotion", "کاهش پویانمایی‌ها");

        en_map.insert("SettingsTransparentScreenshot", "Transparent screenshot background");
        fa_map.insert("SettingsTransparentScreenshot", "تصویربرداری شفاف از صفحه نمایش");

        en_map.insert("SettingsCustomBg", "Custom background image");
        fa_map.insert("SettingsCustomBg", "تصویر پس‌زمینه دلخواه");

        en_map.insert("SettingsChooseImage", "Choose an image file...");
        fa_map.insert("SettingsChooseImage", "انتخاب فایل تصویر...");

        en_map.insert("SettingsBrowse", "Browse...");
        fa_map.insert("SettingsBrowse", "بررسی...");

        en_map.insert("SettingsAppearanceNote", "When desktop blur is active, your screen is captured under the break overlay.\nIf disabled or unavailable, a smooth dark gradient is used.\n\nOptionally pick a custom image to show behind the break overlay.");
        fa_map.insert("SettingsAppearanceNote", "هنگام فعال بودن استراحت، تصویری از صفحه شما در پس‌زمینه قرار می‌گیرد.\nدر صورت غیرفعال بودن، گریدینت تاریک استفاده می‌شود.\nمی‌توانید تصویر دلخواه خود را نیز انتخاب کنید.");

        en_map.insert("SettingsSystemSection", "System & Startup");
        fa_map.insert("SettingsSystemSection", "سیستم و راه‌اندازی");

        en_map.insert("SettingsRunAtStartup", "Run at system startup");
        fa_map.insert("SettingsRunAtStartup", "اجرا هنگام راه‌اندازی سیستم");

        en_map.insert("SettingsStartMinimized", "Start minimized to system tray");
        fa_map.insert("SettingsStartMinimized", "اجرا به صورت کمینه در سینی سیستم");

        en_map.insert("SettingsCloseToTray", "Close dashboard window to tray");
        fa_map.insert("SettingsCloseToTray", "بستن پنجره به سینی سیستم");

        en_map.insert("SettingsSave", "Save Changes");
        fa_map.insert("SettingsSave", "ذخیره تغییرات");

        en_map.insert("SettingsCancel", "Cancel");
        fa_map.insert("SettingsCancel", "انصراف");

        en_map.insert("MinutesUnit", "minutes");
        fa_map.insert("MinutesUnit", "دقیقه");

        en_map.insert("SecondsUnit", "seconds");
        fa_map.insert("SecondsUnit", "ثانیه");

        en_map.insert("HoursUnit", "h");
        fa_map.insert("HoursUnit", "ساعت");

        // Sound Themes Names
        en_map.insert("SoundThemeSoftBell", "Soft Bell");
        fa_map.insert("SoundThemeSoftBell", "زنگ نرم");

        en_map.insert("SoundThemeGentleChime", "Gentle Chime");
        fa_map.insert("SoundThemeGentleChime", "چایم ملایم");

        en_map.insert("SoundThemeCalmDrop", "Calm Drop");
        fa_map.insert("SoundThemeCalmDrop", "قطره آرام");

        en_map.insert("SoundThemeCalmPiano", "Calm Piano");
        fa_map.insert("SoundThemeCalmPiano", "پیانو آرام");

        en_map.insert("SoundThemeMorningDew", "Morning Dew");
        fa_map.insert("SoundThemeMorningDew", "شبنم صبحگاهی");

        en_map.insert("SoundThemeStillWater", "Still Water");
        fa_map.insert("SoundThemeStillWater", "آب زلال");

        // About Window
        en_map.insert("AboutTitle", "About Awayra");
        fa_map.insert("AboutTitle", "درباره Awayra");

        en_map.insert("AboutVersion", "Version 1.0.0 (Linux Rust Port)");
        fa_map.insert("AboutVersion", "نسخه ۱.۰.۰ (مخصوص لینوکس)");

        en_map.insert("AboutDescription", "A calm, non-intrusive break reminder to help reduce eye strain and promote healthy movement habits during long computer sessions.");
        fa_map.insert("AboutDescription", "یک یادآور استراحت آرام و غیرمزاحم برای کاهش خستگی چشم و حفظ سلامت بدن در طول کار با کامپیوتر.");

        en_map.insert("GitHub", "GitHub");
        fa_map.insert("GitHub", "گیت‌هاب");

        en_map.insert("Close", "Close");
        fa_map.insert("Close", "بستن");

        // Validations
        en_map.insert("ValidationEyeResetIntervalInvalid", "Eye reset interval must be between 1 and 480 minutes.");
        fa_map.insert("ValidationEyeResetIntervalInvalid", "فاصله استراحت چشم باید بین ۱ تا ۴۸۰ دقیقه باشد.");

        en_map.insert("ValidationEyeResetDurationInvalid", "Eye reset duration must be between 5 and 600 seconds.");
        fa_map.insert("ValidationEyeResetDurationInvalid", "مدت زمان استراحت چشم باید بین ۵ تا ۶۰۰ ثانیه باشد.");

        en_map.insert("ValidationMoveBreakIntervalInvalid", "Move break interval must be between 1 and 480 minutes.");
        fa_map.insert("ValidationMoveBreakIntervalInvalid", "فاصله استراحت حرکتی باید بین ۱ تا ۴۸۰ دقیقه باشد.");

        en_map.insert("ValidationMoveBreakDurationInvalid", "Move break duration must be between 5 and 600 seconds.");
        fa_map.insert("ValidationMoveBreakDurationInvalid", "مدت زمان استراحت حرکتی باید بین ۵ تا ۶۰۰ ثانیه باشد.");

        en_map.insert("ValidationSnoozeDurationInvalid", "Snooze duration must be between 1 and 60 minutes.");
        fa_map.insert("ValidationSnoozeDurationInvalid", "مدت زمان تعویق باید بین ۱ تا ۶۰ دقیقه باشد.");

        en_map.insert("ValidationIdleThresholdInvalid", "Idle threshold must be between 1 and 120 minutes.");
        fa_map.insert("ValidationIdleThresholdInvalid", "آستانه عدم فعالیت باید بین ۱ تا ۱۲۰ دقیقه باشد.");

        en_map.insert("ValidationGlassClarityInvalid", "Glass clarity must be between 0 and 150.");
        fa_map.insert("ValidationGlassClarityInvalid", "شفافیت شیشه باید بین ۰ تا ۱۵۰ باشد.");

        en_map.insert("ValidationWorkHoursRangeInvalid", "Work start and end cannot be the same time.");
        fa_map.insert("ValidationWorkHoursRangeInvalid", "زمان شروع و پایان کار نمی‌تواند یکسان باشد.");

        Self { en_map, fa_map }
    }

    pub fn get(&self, key: &str, lang: AppLanguage) -> String {
        let map = match lang {
            AppLanguage::English => &self.en_map,
            AppLanguage::Persian => &self.fa_map,
        };
        map.get(key).copied().unwrap_or(key).to_string()
    }
}
