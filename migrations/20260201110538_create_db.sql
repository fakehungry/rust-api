-- Mock data for Task Tracker API
-- This includes tasks table with various states and priorities

-- First, create the tasks table (if not exists)
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP WITH TIME ZONE
);

-- Insert mock tasks
INSERT INTO tasks (title, description, status, priority, due_date) VALUES
('Set up development environment', 'Install Rust, PostgreSQL, and configure IDE with rust-analyzer', 'completed', 'high', '2024-01-15 10:00:00+00'),
('Learn Actix-web basics', 'Go through Actix-web documentation and build a hello world server', 'completed', 'high', '2024-01-16 15:00:00+00'),
('Implement CRUD endpoints', 'Create POST, GET, PUT, DELETE endpoints for tasks resource', 'in_progress', 'high', '2024-01-20 09:00:00+00'),
('Write integration tests', 'Add test cases for all API endpoints using actix-web test utilities', 'pending', 'high', '2024-01-25 17:00:00+00'),
('Add input validation', 'Implement request validation using serde and custom validators', 'pending', 'medium', '2024-01-22 12:00:00+00'),
('Set up database migrations', 'Create SQLx migrations for database schema management', 'completed', 'high', '2024-01-17 11:00:00+00'),
('Implement error handling', 'Create custom error types and proper HTTP error responses', 'in_progress', 'medium', '2024-01-23 14:00:00+00'),
('Add logging and tracing', 'Integrate tracing crate for structured logging', 'pending', 'medium', '2024-01-26 10:00:00+00'),
('Configure CORS', 'Set up CORS middleware for frontend integration', 'pending', 'low', '2024-01-28 16:00:00+00'),
('Create Docker setup', 'Write Dockerfile and docker-compose.yml for local development', 'pending', 'medium', '2024-01-30 09:00:00+00'),
('Implement authentication', 'Add JWT-based authentication for protected endpoints', 'pending', 'high', '2024-02-05 10:00:00+00'),
('Add pagination', 'Implement pagination for GET /tasks endpoint', 'pending', 'medium', '2024-01-27 13:00:00+00'),
('Optimize database queries', 'Add indexes and analyze query performance', 'pending', 'low', '2024-02-10 11:00:00+00'),
('Write API documentation', 'Document all endpoints using OpenAPI/Swagger', 'pending', 'low', '2024-02-15 15:00:00+00'),
('Refactor project structure', 'Organize code into modules: handlers, models, db, config', 'in_progress', 'medium', '2024-01-21 10:00:00+00'),
('Add health check endpoint', 'Implement /health endpoint for monitoring', 'completed', 'medium', '2024-01-18 14:00:00+00'),
('Set up CI/CD pipeline', 'Configure GitHub Actions for testing and deployment', 'pending', 'medium', '2024-02-20 12:00:00+00'),
('Implement rate limiting', 'Add rate limiting middleware to prevent abuse', 'pending', 'low', '2024-02-25 09:00:00+00'),
('Add search functionality', 'Allow filtering tasks by title, status, and priority', 'pending', 'medium', '2024-01-29 11:00:00+00'),
('Deploy to production', 'Deploy the application to a cloud provider', 'pending', 'high', '2024-03-01 10:00:00+00');

-- Create a users table for authentication practice
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- Insert mock users (passwords are hashed version of "password123")
INSERT INTO users (username, email, password_hash, full_name) VALUES
('john_doe', 'john.doe@example.com', '$argon2id$v=19$m=19456,t=2,p=1$hash1', 'John Doe'),
('jane_smith', 'jane.smith@example.com', '$argon2id$v=19$m=19456,t=2,p=1$hash2', 'Jane Smith'),
('bob_wilson', 'bob.wilson@example.com', '$argon2id$v=19$m=19456,t=2,p=1$hash3', 'Bob Wilson'),
('alice_brown', 'alice.brown@example.com', '$argon2id$v=19$m=19456,t=2,p=1$hash4', 'Alice Brown'),
('charlie_davis', 'charlie.davis@example.com', '$argon2id$v=19$m=19456,t=2,p=1$hash5', 'Charlie Davis');

-- Add user_id to tasks table for multi-user support
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS user_id INTEGER REFERENCES users(id);

-- Update existing tasks with random user assignments
UPDATE tasks SET user_id = 1 WHERE id IN (1, 2, 3, 4);
UPDATE tasks SET user_id = 2 WHERE id IN (5, 6, 7, 8);
UPDATE tasks SET user_id = 3 WHERE id IN (9, 10, 11, 12);
UPDATE tasks SET user_id = 4 WHERE id IN (13, 14, 15, 16);
UPDATE tasks SET user_id = 5 WHERE id IN (17, 18, 19, 20);

-- Create categories table for practice with JOINs
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    color VARCHAR(7) -- hex color code
);

INSERT INTO categories (name, description, color) VALUES
('Development', 'Software development tasks', '#3B82F6'),
('Testing', 'Quality assurance and testing', '#10B981'),
('Documentation', 'Writing docs and guides', '#F59E0B'),
('DevOps', 'Deployment and infrastructure', '#8B5CF6'),
('Research', 'Learning and research tasks', '#EC4899');

-- Create junction table for many-to-many relationship
CREATE TABLE IF NOT EXISTS task_categories (
    task_id INTEGER REFERENCES tasks(id) ON DELETE CASCADE,
    category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, category_id)
);

INSERT INTO task_categories (task_id, category_id) VALUES
(1, 1), (2, 1), (3, 1), (4, 2), (5, 1),
(6, 1), (7, 1), (8, 1), (9, 1), (10, 4),
(11, 1), (12, 1), (13, 1), (14, 3), (15, 1),
(16, 4), (17, 4), (18, 1), (19, 1), (20, 4),
(2, 5), (4, 3), (14, 1);

-- Create comments table for practice with one-to-many relationships
CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    task_id INTEGER REFERENCES tasks(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO comments (task_id, user_id, content) VALUES
(3, 1, 'Started working on the POST endpoint, making good progress'),
(3, 2, 'Don''t forget to add proper error handling!'),
(7, 2, 'Implemented custom error types using thiserror crate'),
(15, 3, 'Moved handlers to separate module, much cleaner now'),
(11, 4, 'Should we use JWT or session-based auth?'),
(11, 1, 'JWT would be better for a stateless API'),
(4, 1, 'Need to write tests for edge cases'),
(12, 2, 'Added LIMIT and OFFSET parameters');