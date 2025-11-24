ALTER TABLE
    `user`
ADD
    `name` text NOT NULL;

--> statement-breakpoint
ALTER TABLE
    `user` DROP COLUMN `title`;