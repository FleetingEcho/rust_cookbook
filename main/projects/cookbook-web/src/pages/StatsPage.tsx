import { useNavigate } from '@tanstack/react-router'
import { useStats, useCategories } from '../api/hooks'
import {
  BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer,
  PieChart, Pie, Cell, Legend,
} from 'recharts'
import { Skeleton } from '../components/Skeleton'

const PIE_COLORS = [
  '#8B4513', '#C8691A', '#D4A853', '#6B8C5F',
  '#5C7A9E', '#9B6B9B', '#3a8a9a', '#a06050',
  '#7a7a3a', '#4a6a4a', '#6a4a8a', '#3a5a7a',
]

function StatCard({ value, label, sub }: { value: string | number; label: string; sub?: string }) {
  return (
    <div
      className="rounded-2xl p-6"
      style={{ background: 'white', border: '1px solid var(--color-border)' }}
    >
      <div
        className="text-4xl font-bold mb-1"
        style={{ fontFamily: 'var(--font-display)', color: 'var(--color-rust)', fontStyle: 'italic' }}
      >
        {value}
      </div>
      <div className="text-sm font-medium" style={{ color: 'var(--color-ink)' }}>{label}</div>
      {sub && <div className="text-xs mt-0.5" style={{ color: 'var(--color-ink-muted)' }}>{sub}</div>}
    </div>
  )
}

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null
  return (
    <div
      className="px-3 py-2 rounded-xl text-sm"
      style={{
        background: 'var(--color-ink)',
        color: 'var(--color-paper)',
        boxShadow: 'var(--shadow-card-hover)',
      }}
    >
      <div className="font-medium">{label}</div>
      <div style={{ color: 'var(--color-gold)' }}>{payload[0].value} 道菜</div>
    </div>
  )
}

export function StatsPage() {
  const { data: stats, isLoading } = useStats()
  const { data: categories } = useCategories()
  const navigate = useNavigate()

  function goToCategory(name: string) {
    navigate({ to: '/recipes', search: { category: name } } as never)
  }

  if (isLoading) {
    return (
      <div className="max-w-5xl mx-auto px-8 py-10">
        <Skeleton className="h-9 w-48 mb-8" />
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-10">
          {Array.from({ length: 4 }, (_, i) => <Skeleton key={i} className="h-28 rounded-2xl" />)}
        </div>
        <Skeleton className="h-72 rounded-2xl" />
      </div>
    )
  }

  if (!stats) return null

  const categoryData = Object.entries(stats.by_category)
    .sort((a, b) => b[1] - a[1])
    .map(([name, count]) => ({ name, count }))

  const sourceData = Object.entries(stats.sources)
    .sort((a, b) => b[1] - a[1])
    .map(([name, count], i) => ({ name, count, fill: PIE_COLORS[i % PIE_COLORS.length] }))

  const topCategories = categoryData.slice(0, 12)

  return (
    <div className="max-w-5xl mx-auto px-8 py-10">
      <div className="mb-10">
        <h1
          className="text-3xl font-bold mb-2"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          数据统计
        </h1>
        <p className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
          菜谱库概览
        </p>
      </div>

      {/* KPI row */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-10">
        <StatCard value={stats.total_recipes.toLocaleString()} label="菜谱总数" sub="道" />
        <StatCard
          value={stats.avg_calories ? Math.round(stats.avg_calories) : '—'}
          label="平均热量"
          sub="大卡 / 份"
        />
        <StatCard
          value={Object.keys(stats.by_category).length}
          label="菜品分类"
          sub="个大类"
        />
        <StatCard
          value={Object.keys(stats.sources).length}
          label="数据来源"
          sub="个渠道"
        />
      </div>

      <div className="grid md:grid-cols-2 gap-6 mb-6">
        {/* Bar chart: category distribution */}
        <div
          className="rounded-2xl p-6"
          style={{ background: 'white', border: '1px solid var(--color-border)' }}
        >
          <h2
            className="text-base font-semibold mb-5"
            style={{ fontFamily: 'var(--font-display)' }}
          >
            分类分布
          </h2>
          <ResponsiveContainer width="100%" height={280}>
            <BarChart
              data={topCategories}
              layout="vertical"
              margin={{ left: 0, right: 16 }}
              style={{ cursor: 'pointer' }}
              onClick={d => d?.activePayload?.[0] && goToCategory(d.activePayload[0].payload.name)}
            >
              <XAxis type="number" tick={{ fontSize: 11, fill: 'var(--color-ink-muted)' }} axisLine={false} tickLine={false} />
              <YAxis
                type="category"
                dataKey="name"
                tick={{ fontSize: 11, fill: 'var(--color-ink)' }}
                axisLine={false}
                tickLine={false}
                width={52}
              />
              <Tooltip content={<CustomTooltip />} cursor={{ fill: 'var(--color-paper-dark)' }} />
              <Bar dataKey="count" radius={[0, 6, 6, 0]}>
                {topCategories.map((_, i) => (
                  <Cell
                    key={i}
                    fill={i === 0 ? 'var(--color-rust)' : i < 3 ? 'var(--color-caramel)' : 'var(--color-gold)'}
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Pie chart: sources */}
        <div
          className="rounded-2xl p-6"
          style={{ background: 'white', border: '1px solid var(--color-border)' }}
        >
          <h2
            className="text-base font-semibold mb-5"
            style={{ fontFamily: 'var(--font-display)' }}
          >
            数据来源
          </h2>
          <ResponsiveContainer width="100%" height={280}>
            <PieChart>
              <Pie
                data={sourceData}
                cx="50%"
                cy="45%"
                innerRadius={60}
                outerRadius={100}
                paddingAngle={2}
                dataKey="count"
              >
                {sourceData.map((entry, i) => (
                  <Cell key={i} fill={entry.fill} />
                ))}
              </Pie>
              <Tooltip
                formatter={(v: number) => [`${v} 道`, '数量']}
                contentStyle={{
                  background: 'var(--color-ink)',
                  border: 'none',
                  borderRadius: 12,
                  color: 'var(--color-paper)',
                  fontSize: 12,
                }}
              />
              <Legend
                iconType="circle"
                iconSize={8}
                wrapperStyle={{ fontSize: 11, color: 'var(--color-ink-muted)' }}
              />
            </PieChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Full category table */}
      <div
        className="rounded-2xl overflow-hidden"
        style={{ border: '1px solid var(--color-border)' }}
      >
        <div
          className="px-6 py-4"
          style={{ background: 'var(--color-ink)' }}
        >
          <h2
            className="text-base font-semibold italic"
            style={{ fontFamily: 'var(--font-display)', color: 'var(--color-paper)' }}
          >
            全部分类
          </h2>
        </div>
        <div className="divide-y" style={{ borderColor: 'var(--color-border)', background: 'white' }}>
          {categoryData.map((c, i) => {
            const pct = Math.round((c.count / stats.total_recipes) * 100)
            return (
              <button
                key={c.name}
                onClick={() => goToCategory(c.name)}
                className="w-full flex items-center gap-4 px-6 py-3 text-left transition-colors hover:bg-[var(--color-paper)]"
                title={`查看${c.name}菜谱`}
              >
                <span
                  className="text-sm w-5 text-right shrink-0"
                  style={{ color: 'var(--color-ink-muted)', fontFamily: 'var(--font-display)', fontStyle: 'italic' }}
                >
                  {i + 1}
                </span>
                <span className="text-sm font-medium flex-1 text-left" style={{ color: 'var(--color-ink)' }}>
                  {c.name}
                </span>
                <div className="w-32 h-1.5 rounded-full overflow-hidden" style={{ background: 'var(--color-paper-dark)' }}>
                  <div
                    className="h-full rounded-full"
                    style={{
                      width: `${pct}%`,
                      background: i === 0 ? 'var(--color-rust)' : i < 3 ? 'var(--color-caramel)' : 'var(--color-gold)',
                    }}
                  />
                </div>
                <span className="text-sm w-12 text-right" style={{ color: 'var(--color-ink-muted)' }}>
                  {c.count}
                </span>
                <span className="text-xs w-10 text-right" style={{ color: 'var(--color-ink-muted)' }}>
                  {pct}%
                </span>
              </button>
            )
          })}
        </div>
      </div>
    </div>
  )
}
