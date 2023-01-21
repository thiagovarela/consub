INSERT INTO accounts.accounts (id,name,subdomain) VALUES
	 ('263b6188-aac6-45e9-9a2c-4728fdfd7ea1','Consub','consub');

INSERT INTO accounts.account_keys (id,account_id,public_key,secret_key,expires_at) VALUES
	 ('e6af50e6-0ef3-4908-80c0-a83622d96d03','263b6188-aac6-45e9-9a2c-4728fdfd7ea1',decode('8C3AE22F7B8A7FEBFB105A8AF419D6B0167C991B69D6F0F82CBA688B58BEBC49','hex'),decode('38196E146B15FD60D1B45E85598E53A72E0C669F7B770C93D5F715ABD88886A6','hex'),NULL);

INSERT INTO accounts.users (id,account_id) VALUES
	 ('ad38ffbe-dabe-43a1-b63a-4028e23090eb','263b6188-aac6-45e9-9a2c-4728fdfd7ea1');

-- password: 123456
INSERT INTO accounts.passwords (user_id,account_id,email,hash_password) VALUES
	 ('ad38ffbe-dabe-43a1-b63a-4028e23090eb','263b6188-aac6-45e9-9a2c-4728fdfd7ea1','thiagovarela@consub.io','$argon2id$v=19$m=4096,t=3,p=1$jbrAa6jwSsVfuIT8Fys3ew$g7y2yWMX/Lq9WGiwCo210v5so7qX8Eo6X67PpJCACT4');

