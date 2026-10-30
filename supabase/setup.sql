-- Taskal 2: one-time setup for accounts and sync.
-- Run this once in Supabase: SQL Editor -> New query -> paste -> Run.

-- One row per person holding their Taskal data (tasks, courses, lecture marks, notes).
-- The calendar link is never uploaded; the app strips it before syncing.
create table if not exists public.taskal_data (
  user_id    uuid primary key references auth.users (id) on delete cascade,
  data       jsonb not null,
  updated_at timestamptz not null default now()
);

-- Row level security: each person can only see and change their own row.
alter table public.taskal_data enable row level security;

drop policy if exists "own row: read"   on public.taskal_data;
drop policy if exists "own row: insert" on public.taskal_data;
drop policy if exists "own row: update" on public.taskal_data;
drop policy if exists "own row: delete" on public.taskal_data;

create policy "own row: read"   on public.taskal_data for select using (auth.uid() = user_id);
create policy "own row: insert" on public.taskal_data for insert with check (auth.uid() = user_id);
create policy "own row: update" on public.taskal_data for update using (auth.uid() = user_id) with check (auth.uid() = user_id);
create policy "own row: delete" on public.taskal_data for delete using (auth.uid() = user_id);

-- "Delete my account": removes the signed-in person's login and, through the cascade, their synced data.
create or replace function public.delete_my_account()
returns void
language sql
security definer
set search_path = ''
as $$
  delete from auth.users where id = auth.uid();
$$;

revoke all on function public.delete_my_account() from public, anon;
grant execute on function public.delete_my_account() to authenticated;
