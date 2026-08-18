-- Meeting tables
CREATE TABLE IF NOT EXISTS x_meeting_building (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address TEXT,
    description TEXT,
    order_number INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_meeting_room (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    building_id VARCHAR(255),
    floor VARCHAR(50),
    capacity INTEGER DEFAULT 0,
    equipment JSONB DEFAULT '{}',
    description TEXT,
    photo TEXT,
    open_meeting BOOLEAN DEFAULT FALSE,
    order_number INTEGER DEFAULT 0,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_meeting (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    content TEXT,
    room_id VARCHAR(255),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    creator VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS x_meeting_invite (
    id VARCHAR(255) PRIMARY KEY,
    meeting_id VARCHAR(255) NOT NULL,
    invitee VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'wait',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_meeting_room_building ON x_meeting_room(building_id);
CREATE INDEX IF NOT EXISTS idx_meeting_invite_meeting ON x_meeting_invite(meeting_id);
CREATE INDEX IF NOT EXISTS idx_meeting_creator ON x_meeting(creator);
