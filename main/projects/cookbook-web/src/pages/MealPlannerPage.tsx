import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import { CalendarDays, RefreshCw, Users, Sun, Sunset, Moon } from 'lucide-react'
import { useMealPlan } from '../api/hooks'
import type { DayPlan, RecipeSummary } from '../api/types'

const DAYS_OPTIONS = [3, 5, 7, 10, 14]

export function MealPlannerPage() {
  const [days, setDays] = useState(7)
  const [people, setPeople] = useState(2)
  const [maxCal, setMaxCal] = useState<number | undefined>()
  const [maxDiff, setMaxDiff] = useState<number | undefined>()
  const { mutate, data, isPending, error } = useMealPlan()

  function generate() {
    mutate({ days, people, max_calories_per_meal: maxCal, max_difficulty: maxDiff })
  }

  return (
    <div className="max-w-5xl mx-auto px-8 py-10">
      <div className="mb-8">
        <h1
          className="text-3xl font-bold mb-2"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          每周餐单
        </h1>
        <p className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
          自动生成每日早午晚三餐计划
        </p>
      </div>

      {/* Config card */}
      <div
        className="rounded-2xl p-6 mb-8"
        style={{
          background: 'white',
          border: '1px solid var(--color-border)',
          boxShadow: 'var(--shadow-card)',
        }}
      >
        <div className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-6">
          {/* Days */}
          <div>
            <label className="block text-xs font-medium mb-2" style={{ color: 'var(--color-ink-muted)' }}>
              天数
            </label>
            <div className="flex gap-1.5 flex-wrap">
              {DAYS_OPTIONS.map(d => (
                <button
                  key={d}
                  onClick={() => setDays(d)}
                  className="px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
                  style={{
                    background: days === d ? 'var(--color-ink)' : 'var(--color-paper)',
                    color: days === d ? 'var(--color-paper)' : 'var(--color-ink-muted)',
                    border: `1px solid ${days === d ? 'var(--color-ink)' : 'var(--color-border)'}`,
                  }}
                >
                  {d} 天
                </button>
              ))}
            </div>
          </div>

          {/* People */}
          <div>
            <label className="block text-xs font-medium mb-2" style={{ color: 'var(--color-ink-muted)' }}>
              <Users size={11} className="inline mr-1" />
              用餐人数
            </label>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setPeople(p => Math.max(1, p - 1))}
                className="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-bold transition-colors"
                style={{ background: 'var(--color-paper)', border: '1px solid var(--color-border)' }}
              >
                −
              </button>
              <span className="text-lg font-bold w-6 text-center" style={{ fontFamily: 'var(--font-display)' }}>
                {people}
              </span>
              <button
                onClick={() => setPeople(p => Math.min(10, p + 1))}
                className="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-bold transition-colors"
                style={{ background: 'var(--color-paper)', border: '1px solid var(--color-border)' }}
              >
                +
              </button>
            </div>
          </div>

          {/* Max calories */}
          <div>
            <label className="block text-xs font-medium mb-2" style={{ color: 'var(--color-ink-muted)' }}>
              每餐热量上限（大卡）
            </label>
            <div className="flex gap-1.5 flex-wrap">
              {[undefined, 300, 500, 800].map(c => (
                <button
                  key={String(c)}
                  onClick={() => setMaxCal(c)}
                  className="px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
                  style={{
                    background: maxCal === c ? 'var(--color-ink)' : 'var(--color-paper)',
                    color: maxCal === c ? 'var(--color-paper)' : 'var(--color-ink-muted)',
                    border: `1px solid ${maxCal === c ? 'var(--color-ink)' : 'var(--color-border)'}`,
                  }}
                >
                  {c ? `≤${c}` : '不限'}
                </button>
              ))}
            </div>
          </div>

          {/* Max difficulty */}
          <div>
            <label className="block text-xs font-medium mb-2" style={{ color: 'var(--color-ink-muted)' }}>
              难度上限
            </label>
            <div className="flex gap-1.5 flex-wrap">
              {[undefined, 2, 3, 5].map(d => (
                <button
                  key={String(d)}
                  onClick={() => setMaxDiff(d)}
                  className="px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
                  style={{
                    background: maxDiff === d ? 'var(--color-ink)' : 'var(--color-paper)',
                    color: maxDiff === d ? 'var(--color-paper)' : 'var(--color-ink-muted)',
                    border: `1px solid ${maxDiff === d ? 'var(--color-ink)' : 'var(--color-border)'}`,
                  }}
                >
                  {d ? '★'.repeat(d) : '不限'}
                </button>
              ))}
            </div>
          </div>
        </div>

        <button
          onClick={generate}
          disabled={isPending}
          className="flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-opacity hover:opacity-90 disabled:opacity-60"
          style={{ background: 'var(--color-ink)', color: 'var(--color-paper)' }}
        >
          {isPending
            ? <><div className="w-4 h-4 rounded-full border-2 border-paper/40 border-t-paper animate-spin" /> 生成中…</>
            : <><CalendarDays size={15} />{data ? '重新生成' : '生成餐单'}</>
          }
        </button>

        {error && (
          <p className="mt-3 text-sm" style={{ color: '#c0392b' }}>
            {(error as Error).message}
          </p>
        )}
      </div>

      {/* Meal plan grid */}
      {data && (
        <div className="space-y-5">
          <div className="flex items-center gap-3 mb-6">
            <span className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
              {data.people} 人 · {data.days.length} 天餐单
            </span>
            <button
              onClick={generate}
              className="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-full transition-opacity hover:opacity-70"
              style={{ color: 'var(--color-rust)', border: '1px solid var(--color-rust-light)', background: 'var(--color-rust-light)' }}
            >
              <RefreshCw size={11} />
              换一套
            </button>
          </div>

          {data.days.map(day => <DayCard key={day.day} day={day} />)}
        </div>
      )}

      {/* Empty prompt */}
      {!data && !isPending && (
        <div className="text-center py-20">
          <CalendarDays size={48} className="mx-auto mb-4 opacity-20" style={{ color: 'var(--color-ink)' }} />
          <p className="text-base" style={{ color: 'var(--color-ink-muted)' }}>
            设置偏好后点击「生成餐单」
          </p>
        </div>
      )}
    </div>
  )
}

function DayCard({ day }: { day: DayPlan }) {
  return (
    <div
      className="rounded-2xl overflow-hidden"
      style={{ border: '1px solid var(--color-border)' }}
    >
      {/* Day header */}
      <div
        className="px-5 py-3 flex items-center gap-3"
        style={{ background: 'var(--color-ink)' }}
      >
        <span
          className="text-base font-bold italic"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-gold)' }}
        >
          第 {day.day} 天
        </span>
      </div>

      <div className="grid md:grid-cols-3 divide-x" style={{ borderColor: 'var(--color-border)', background: 'white' }}>
        <MealSlot icon={<Sun size={13} />} label="早餐" recipes={[day.breakfast]} />
        <MealSlot icon={<Sunset size={13} />} label="午餐" recipes={day.lunch} />
        <MealSlot icon={<Moon size={13} />} label="晚餐" recipes={day.dinner} />
      </div>
    </div>
  )
}

function MealSlot({
  icon, label, recipes,
}: { icon: React.ReactNode; label: string; recipes: RecipeSummary[] }) {
  return (
    <div className="p-4 flex flex-col gap-3">
      <div className="flex items-center gap-1.5 text-xs font-medium" style={{ color: 'var(--color-ink-muted)' }}>
        {icon}{label}
      </div>
      <div className="flex flex-col gap-2">
        {recipes.map(r => (
          <Link
            key={r.id}
            to="/recipes/$id"
            params={{ id: String(r.id) }}
            className="flex items-center gap-2.5 p-2 rounded-xl transition-colors hover:bg-[var(--color-paper)]"
          >
            {r.cover_image
              ? <img src={r.cover_image} alt={r.name} className="w-10 h-10 rounded-lg object-cover shrink-0" />
              : (
                <div
                  className="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0"
                  style={{ background: 'var(--color-paper-dark)' }}
                >
                  🍽
                </div>
              )
            }
            <div className="min-w-0">
              <p className="text-sm font-medium truncate" style={{ color: 'var(--color-ink)' }}>
                {r.name}
              </p>
              <p className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>
                {r.calories ? `${Math.round(r.calories)} 大卡 · ` : ''}{r.category}
              </p>
            </div>
          </Link>
        ))}
      </div>
    </div>
  )
}
