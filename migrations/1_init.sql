create table source (
    id text primary key,
    link text unique, -- load proxies from this link
    default_protocol text check (default_protocol in ('http', 'socks5')),
    default_username text,
    default_password text,
    default_port integer,
    items text[] not null default '{}', -- item can be a proxy url or an ip
    proxy_count integer not null default 0, -- number of proxies in the source
    created_at timestamptz not null default now(),
    checked_at timestamptz
);
create index on source(created_at);
create index on source(checked_at);

create table proxy (
    id bigserial primary key,
    status text not null check (status in ('unknown', 'ok', 'down')) default 'unknown',
    source_id text not null references source(id),
    url text not null,
    ip text not null unique,
    protocol text not null check (protocol in ('http', 'socks5')),
    created_at timestamptz not null default now(),
    checked_at timestamptz,
    last_ok_at timestamptz,
    check_history boolean[] not null default '{}'
);
create index on proxy(status);
create index on proxy(source_id);
create index on proxy(protocol);
create index on proxy(created_at);
create index on proxy(checked_at);
create index on proxy(last_ok_at);
