import { Link, useLocation } from '@tanstack/react-router'
import { Search, BookOpen, Salad, CalendarDays, BarChart3 } from 'lucide-react'
import type { ReactNode } from 'react'

const NAV = [
  { to: '/recipes',        label: '菜谱',   icon: BookOpen },
  { to: '/by-ingredients', label: '以材查菜', icon: Salad },
  { to: '/meal-planner',   label: '餐单',   icon: CalendarDays },
  { to: '/stats',          label: '统计',   icon: BarChart3 },
]

export function RootLayout({ children }: { children: ReactNode }) {
  const loc = useLocation()

  return (
    <div className="flex flex-col min-h-screen">
      {/* ── Nav ─────────────────────────────────────────────────────── */}
      <header
        className="sticky top-0 z-50 flex items-center gap-6 px-8 py-0 h-14"
        style={{ background: 'var(--color-ink)' }}
      >
        <Link to="/" className="shrink-0">
          <span
            className="text-lg font-semibold italic tracking-tight"
            style={{ fontFamily: 'var(--font-display)', color: 'var(--color-paper)' }}
          >
            菜谱
          </span>
        </Link>

        <nav className="flex items-center gap-1 flex-1">
          {NAV.map(({ to, label, icon: Icon }) => {
            const active = loc.pathname.startsWith(to)
            return (
              <Link
                key={to}
                to={to}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"
                style={{
                  color: active ? 'var(--color-gold)' : 'rgba(250,247,242,0.6)',
                  background: active ? 'rgba(250,247,242,0.08)' : 'transparent',
                }}
              >
                <Icon size={13} />
                {label}
              </Link>
            )
          })}
        </nav>

        <Link
          to="/search"
          className="flex items-center gap-2 px-3 py-1.5 rounded-full text-xs transition-colors"
          style={{
            background: 'rgba(250,247,242,0.1)',
            border: '1px solid rgba(250,247,242,0.15)',
            color: 'rgba(250,247,242,0.55)',
          }}
        >
          <Search size={12} />
          <span>搜索菜谱…</span>
          <kbd
            className="ml-1 text-[10px] px-1.5 py-0.5 rounded"
            style={{ background: 'rgba(250,247,242,0.1)', color: 'rgba(250,247,242,0.4)' }}
          >
            ⌘K
          </kbd>
        </Link>
      </header>

      {/* ── Page content ────────────────────────────────────────────── */}
      <main className="flex-1">
        {children}
      </main>

      {/* ── Footer ──────────────────────────────────────────────────── */}
      <footer
        className="text-center text-xs py-6"
        style={{ color: 'var(--color-ink-muted)', borderTop: '1px solid var(--color-border)' }}
      >
        菜谱 Cookbook · 收录约 700 道家常菜谱
      </footer>
    </div>
  )
}
