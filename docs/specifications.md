## DevQuest Specifications

### Key Features

- **Task Management** - Admin-controlled task creation
- **Point System** - Earn verifiable points for task completion
- **Leaderboard** - Compete with developers globally
- **Project Showcase** - Display your best work
- **On-Chain Records** - All data immutable on blockchain

**Technical Stack:**
- **Blockchain**: Solana
- **Framework**: Anchor (Rust)
- **Frontend**: Next.js + TypeScript
- **Wallet**: Solana Wallet Adapter
- **Anchor SDK**: @coral-xyz/anchor
- **RPC**: Solana Web3.js

## User Stories

### Epic 1: User Authentication & Profile Management

#### US-1.1: Initialize User Profile
```
As a new user
I want to create my profile with name, bio, and GitHub username
So that I can start completing tasks and earning points
```

**Acceptance Criteria:**
- User connects Solana wallet
- User provides name (1-32 chars), bio (max 200 chars), GitHub username (max 32 chars)
- Profile created on-chain with 0 initial points
- User can view their profile immediately after creation
- Profile data persists on blockchain permanently

**Implementation:** `initialize_user` instruction

---

#### US-1.2: Update User Profile
```
As a registered user
I want to update my name, bio, or GitHub username
So that my profile stays updated
```

**Acceptance Criteria:**
- Only profile owner can update their profile
- Fields can be updated individually or all together
- Changes reflected on-chain immediately
- Character limits enforced (name: 32, bio: 200, GitHub: 39)
- Update doesn't affect existing points or project count

**Implementation:** `update_user_account` instruction

---

### Epic 2: Task Management

#### US-2.1: Create Task (Admin)
```
As an admin
I want to create new coding tasks with title, description, difficulty, and point rewards
So that developers have challenges to complete
```

**Acceptance Criteria:**
- Only admin wallet can create tasks
- Task includes: title, description, difficulty (beginner/intermediate/advanced), points reward
- Task ID auto-increments for uniqueness
- Tasks start as active by default
- Task data immutable once created

**Implementation:** `create_task` instruction

---

#### US-2.2: Submit Task Completion
```
As a developer
I want to submit my GitHub repository link to complete a task
So that I can earn points
```

**Acceptance Criteria:**
- User selects an active task
- User provides valid GitHub repository URL
- Task submission creates unique on-chain record
- Users cannot submit same task twice (duplicate prevention)
- User's total points increase by task reward amount
- Submission timestamp recorded on-chain
- Submission immutable after creation

**Implementation:** `submit_task` instruction

---

#### US-2.3: Update Task Status (Admin)
```
As an admin
I want to activate or deactivate tasks
So that I can control which tasks are available
```

**Acceptance Criteria:**
- Only admin wallet can update task status
- Inactive tasks cannot accept new submissions
- Existing submissions remain valid
- Status change reflected immediately
- Admin can reactivate deactivated tasks

**Implementation:** `update_task_status` instruction

---

### Epic 3: Project Showcase

#### US-3.1: Add Project
```
As a user
I want to showcase my projects with name, description, repo link, and website
So that others can see my best work
```

**Acceptance Criteria:**
- User provides: name (max 64 chars), description (max 300 chars), repo URL, website URL (optional)
- Project counter increments (maximum 255 projects per user)
- Project creation timestamp recorded
- Projects displayed on user profile
- Project data immutable after creation

**Implementation:** `add_project` instruction

---

### Epic 4: Leaderboard & Discovery

#### US-4.1: View Leaderboard
```
As a user
I want to see top performers ranked by points
So that I can track my progress against others
```

**Acceptance Criteria:**
- Leaderboard shows all users sorted by total points (descending)
- Displays: rank, username, total points, tasks completed
- Current user's rank highlighted
- Top performers easily identifiable

**Frontend Feature** (fetches from blockchain)

---

#### US-4.2: View User Profile
```
As any visitor
I want to view another developer's profile
So that I can see their achievements and projects
```

**Acceptance Criteria:**
- Profile displays: name, bio, GitHub link, total points, project count
- Shows completed tasks with timestamps
- Displays showcase projects with repos and websites
- Profile accessible via wallet address
- Public, read-only access for all visitors


## Data Models

### UserAccount
- **PDA:** `["user", user_wallet]`
- **Fields:**
  - `owner: PublicKey` - User wallet address
  - `name: String` (max 32) - Display name
  - `bio: String` (max 200) - User bio
  - `github_username: String` (max 32) - GitHub handle
  - `total_points: u64` - **THE COUNTER** - Total points earned
  - `project_counter: u8` - Number of projects (max 255)

### Task
- **PDA:** `["task", task_id]`
- **Fields:**
  - `task_id: u64` - Unique identifier
  - `title: String` - Task title
  - `description: String` - Task description
  - `dificulty: String` - beginner/intermediate/advanced
  - `points_reward: u32` - Points awarded on completion
  - `is_active: bool` - Whether task accepts submissions

### TaskSubmission
- **PDA:** `["submission", user_wallet, task_id]`
- **Fields:**
  - `user: PublicKey` - Submitter wallet
  - `task_id: u64` - Task ID
  - `repo_name: String` - GitHub repository name
  - `submitted_at: i64` - Unix timestamp
  - **Prevents duplicates:** Same user cannot submit same task twice

### Project
- **PDA:** `["project", user_wallet, project_counter]`
- **Fields:**
  - `user: PublicKey` - Project owner
  - `project_id: u8` - Project number (0-9)
  - `name: String` (max 64) - Project name
  - `description: String` (max 300) - Project description
  - `repo_name: String` - GitHub repository name
  - `website_url: String` (optional) - website
  - `created_at: i64` - Creation timestamp



## Instructions

### User Instructions

| Instruction | Description | Signer |
|-------------|-------------|--------|
| `initialize_user` | Create user profile | User |
| `update_user_account` | Update name, bio, GitHub | User |
| `submit_task` | Submit task completion | User |
| `add_project` | Add project to showcase | User |

### Admin Instructions

| Instruction | Description | Signer |
|-------------|-------------|--------|
| `create_task` | Create new task | Admin |
| `update_task_status` | Activate/deactivate task | Admin |

### Setup (One-time)

| Instruction | Description | Signer |
|-------------|-------------|--------|
| `initialize_config` | Setup program config | Admin |


