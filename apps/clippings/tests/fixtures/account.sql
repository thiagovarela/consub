INSERT INTO accounts.accounts (id,name,subdomain) VALUES
	 ('263b6188-aac6-45e9-9a2c-4728fdfd7ea1','Consub','consub');

INSERT INTO accounts.account_keys (id,account_id,keypair,expires_at) VALUES
	 ('e6af50e6-0ef3-4908-80c0-a83622d96d03','263b6188-aac6-45e9-9a2c-4728fdfd7ea1',decode('D3BC1571AE014104D3D758003F1957F364FFC4A476B454B38C365C1717E9B7975AACA9114E01FA97C32A7721F763969CDE613141D27C2047BB80005978FD1658','hex'),NULL);

INSERT INTO accounts.users (id,account_id,email) VALUES
	 ('ad38ffbe-dabe-43a1-b63a-4028e23090eb','263b6188-aac6-45e9-9a2c-4728fdfd7ea1', 'thiagovarela@consub.io');

-- password: 123456
INSERT INTO accounts.passwords (user_id,hash_password) VALUES
	 ('ad38ffbe-dabe-43a1-b63a-4028e23090eb','$argon2id$v=19$m=4096,t=3,p=1$jbrAa6jwSsVfuIT8Fys3ew$g7y2yWMX/Lq9WGiwCo210v5so7qX8Eo6X67PpJCACT4');

