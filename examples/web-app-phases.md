# Example: Web Application Phase Breakdown

This example shows how to break down a typical web application into phases using the Agentic Development Process.

---

## Project: SaaS Dashboard Application

**Goal:** Build a multi-tenant SaaS dashboard with authentication, data visualization, and team collaboration features.

---

## Phase Overview

| Phase | Name | Tasks | Dependencies | Parallel? |
|-------|------|-------|--------------|-----------|
| 1 | Foundation | 7 | None | No |
| 2 | Authentication | 6 | Phase 1 | No |
| 3 | Dashboard Core | 8 | Phase 2 | No |
| 4 | Data Visualization | 5 | Phase 3 | Yes (with 5) |
| 5 | Team Features | 6 | Phase 3 | Yes (with 4) |
| 6 | Notifications | 4 | Phase 4, 5 | No |
| 7 | Polish & Launch | 5 | Phase 6 | No |

**Total Tasks:** 41
**Parallel Opportunities:** Phases 4 & 5 can run simultaneously

---

## Phase 1: Foundation (7 tasks)

**Goal:** Set up project infrastructure and development environment

### Tasks

1. **Project Scaffolding**
   - Initialize Next.js/React project
   - Configure TypeScript
   - Set up ESLint/Prettier

2. **Database Setup**
   - Configure Prisma ORM
   - Design initial schema
   - Set up migrations

3. **Environment Configuration**
   - Create .env structure
   - Set up environment validation
   - Configure for dev/staging/prod

4. **API Framework**
   - Set up API routes structure
   - Configure CORS
   - Add rate limiting

5. **Testing Infrastructure**
   - Configure Jest/Vitest
   - Set up test utilities
   - Create first smoke test

6. **CI/CD Pipeline**
   - GitHub Actions workflow
   - Automated testing on PR
   - Deploy preview environments

7. **Documentation Structure**
   - Create docs folder
   - Add README
   - Set up .coordination structure

### Deliverables
- Working development environment
- Database connected and migrating
- CI/CD running on commits
- Test infrastructure ready

---

## Phase 2: Authentication (6 tasks)

**Goal:** Implement secure user authentication and session management

### Tasks

1. **User Model**
   - Create User schema
   - Add email/password fields
   - Set up password hashing

2. **Registration Flow**
   - Sign-up API endpoint
   - Email validation
   - Welcome email (optional)

3. **Login Flow**
   - Login API endpoint
   - Session/JWT management
   - Remember me functionality

4. **Password Reset**
   - Forgot password flow
   - Reset token generation
   - Password update endpoint

5. **Session Management**
   - Middleware for auth check
   - Token refresh logic
   - Logout functionality

6. **Auth UI Components**
   - Login form
   - Registration form
   - Password reset forms

### Deliverables
- Users can register and login
- Sessions persist correctly
- Password reset works
- Auth middleware protects routes

---

## Phase 3: Dashboard Core (8 tasks)

**Goal:** Build the main dashboard layout and navigation

### Tasks

1. **Layout Components**
   - Sidebar navigation
   - Top header bar
   - Main content area

2. **Navigation System**
   - Route configuration
   - Active state handling
   - Breadcrumbs

3. **User Profile**
   - Profile view page
   - Edit profile form
   - Avatar upload

4. **Settings Pages**
   - Account settings
   - Preferences
   - Security settings

5. **Tenant/Workspace Model**
   - Multi-tenant schema
   - Workspace switching
   - Workspace settings

6. **Dashboard Home**
   - Welcome/overview page
   - Quick actions
   - Recent activity

7. **Search Functionality**
   - Global search bar
   - Search API
   - Results display

8. **Error Handling**
   - Error boundaries
   - 404/500 pages
   - Toast notifications

### Deliverables
- Complete dashboard shell
- Navigation working
- User can manage profile
- Multi-tenant structure in place

---

## Phase 4: Data Visualization (5 tasks)

**Goal:** Add charts, graphs, and data display components

**Note:** Can run in parallel with Phase 5

### Tasks

1. **Chart Library Setup**
   - Install charting library
   - Create chart components
   - Configure themes

2. **Data Fetching Layer**
   - API endpoints for metrics
   - Data aggregation logic
   - Caching strategy

3. **Dashboard Widgets**
   - KPI cards
   - Line/bar charts
   - Pie/donut charts

4. **Date Range Picker**
   - Date range component
   - Relative ranges (7d, 30d, etc.)
   - Custom range selection

5. **Export Functionality**
   - Export to CSV
   - Export to PDF
   - Scheduled reports (optional)

### Deliverables
- Charts render correctly
- Data updates in real-time
- Users can filter by date
- Export works

---

## Phase 5: Team Features (6 tasks)

**Goal:** Implement team collaboration functionality

**Note:** Can run in parallel with Phase 4

### Tasks

1. **Team Model**
   - Team/member schema
   - Role definitions
   - Permissions system

2. **Invite System**
   - Invite API endpoint
   - Email invitations
   - Accept/decline flow

3. **Team Management UI**
   - Team members list
   - Role assignment
   - Remove members

4. **Activity Feed**
   - Activity logging
   - Feed display
   - Filtering options

5. **Comments/Notes**
   - Comment model
   - Comment UI
   - Mentions support

6. **Shared Resources**
   - Sharing permissions
   - Share dialog
   - Shared items view

### Deliverables
- Team invites work
- Roles control access
- Activity is tracked
- Collaboration features work

---

## Phase 6: Notifications (4 tasks)

**Goal:** Add notification system for user engagement

### Tasks

1. **Notification Model**
   - Notification schema
   - Types/categories
   - Read/unread state

2. **In-App Notifications**
   - Notification bell icon
   - Dropdown list
   - Mark as read

3. **Email Notifications**
   - Email templates
   - Send on events
   - Unsubscribe handling

4. **Notification Preferences**
   - Settings UI
   - Per-type toggles
   - Frequency options

### Deliverables
- Users receive notifications
- Email notifications send
- Preferences respected

---

## Phase 7: Polish & Launch (5 tasks)

**Goal:** Final polish and production readiness

### Tasks

1. **Performance Optimization**
   - Bundle analysis
   - Code splitting
   - Image optimization

2. **Accessibility Audit**
   - WCAG compliance check
   - Keyboard navigation
   - Screen reader testing

3. **Security Audit**
   - Dependency audit
   - Penetration testing
   - Fix vulnerabilities

4. **Documentation**
   - User guide
   - API documentation
   - Deployment guide

5. **Launch Checklist**
   - Production environment
   - Monitoring setup
   - Backup procedures

### Deliverables
- Production-ready application
- Documentation complete
- Monitoring in place
- Ready for users

---

## Worktree Setup Commands

```bash
# From main project directory
cd /path/to/saas-dashboard

# Create all phase worktrees
git worktree add ../saas-phase-1 -b feature/phase-1-foundation
git worktree add ../saas-phase-2 -b feature/phase-2-authentication
git worktree add ../saas-phase-3 -b feature/phase-3-dashboard-core
git worktree add ../saas-phase-4 -b feature/phase-4-data-viz
git worktree add ../saas-phase-5 -b feature/phase-5-team-features
git worktree add ../saas-phase-6 -b feature/phase-6-notifications
git worktree add ../saas-phase-7 -b feature/phase-7-polish

# List worktrees
git worktree list
```

---

## Parallel Development Strategy

```
Week 1: Phase 1 (Foundation)
Week 2: Phase 2 (Auth)
Week 3: Phase 3 (Dashboard Core)
Week 4: Phase 4 + Phase 5 (PARALLEL)
        ├── Agent A: Data Visualization
        └── Agent B: Team Features
Week 5: Phase 6 (Notifications)
Week 6: Phase 7 (Polish)
```

**Time Saved:** ~1 week by running Phases 4 & 5 in parallel

---

## Quality Metrics Targets

| Metric | Target |
|--------|--------|
| Test Coverage | >80% |
| Build Time | <2 min |
| Lighthouse Score | >90 |
| Phase Review Score | >8.0/10 |
| Bugs in Develop | 0 |

---

## Notes

- Phases 4 & 5 have no dependencies on each other, so they can be developed simultaneously by different agents
- Phase 6 depends on both 4 & 5, so it must wait for both to merge
- Each phase should take roughly 1 week with focused effort
- Adjust phase boundaries based on your specific requirements
