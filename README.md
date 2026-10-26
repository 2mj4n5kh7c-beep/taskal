# Taskal

Course tasks and a weekly lecture tracker for university students.

- **Tasks** sync from your university's course calendar (Moodle, Canvas and anything else that publishes an iCal link). New assignments appear and changed deadlines update on their own.
- **Lectures**: your weekly timetable for every week of the semester. Mark each session as seen, live or later, and add notes like "seen till 00:57".
- **Semesters** with holiday presets for Israeli universities, so holidays show as "no class".

## Get it

- **Windows / Mac app** (automatic calendar sync): [latest release](https://github.com/2mj4n5kh7c-beep/taskal/releases/latest)
- **Web version** (try it in a browser, import calendar files by hand): https://2mj4n5kh7c-beep.github.io/taskal/

Your data stays on your own device.

## Install on Windows

1. Download **`Taskal_x.y.z_x64-setup.exe`** from the [latest release](https://github.com/2mj4n5kh7c-beep/taskal/releases/latest).
2. Open it. Windows may show **“Windows protected your PC”** because Taskal is new and not yet signed. Click **More info → Run anyway**.
3. Taskal installs for your user only (no admin password) and opens.

## First launch

A short guide walks you through three steps:

1. **Courses**: paste your Moodle calendar link (or choose a calendar file). Taskal lists the courses it finds; untick anything you don’t take.
2. **Tasks**: Taskal shows the deadlines from those courses and adds them. It keeps checking for new ones every few hours.
3. **Timetable**: add when each course meets, and when the semester starts. Israeli holidays are marked as no-class days.

## Connecting your calendar

In Moodle: **Calendar → Import or export calendars → Export calendar → All courses, Recent and next 60 days → Get calendar URL**. Paste that link into Taskal. Many universities open a new Moodle site every year, so you may need a new link each October.

## For developers

The app is a single page in `app/index.html`. `src-tauri/` wraps it as a desktop app (Tauri 2) and adds a native `fetch_calendar` command so calendar links can be read without browser CORS limits.

- Run locally: `npm install && npx tauri dev`
- Release: bump the version in `src-tauri/tauri.conf.json`, then run the **Build desktop apps** workflow. GitHub Actions builds the Windows installer and the Mac app.
