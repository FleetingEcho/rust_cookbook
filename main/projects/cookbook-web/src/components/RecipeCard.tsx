import { Link } from '@tanstack/react-router'
import { Flame, ChefHat } from 'lucide-react'
import type { RecipeSummary } from '../api/types'

const CATEGORY_COLORS: Record<string, string> = {
  荤菜: '#C8691A', 素菜: '#6B8C5F', 早餐: '#D4A853', 汤品: '#5C7A9E',
  甜点: '#9B6B9B', 水产: '#3a8a9a', 主食: '#8B4513', 炒菜: '#C8691A',
}

function categoryColor(cat: string) {
  return CATEGORY_COLORS[cat] ?? '#8a7a6a'
}

function DifficultyDots({ value }: { value: number | null }) {
  if (!value) return null
  return (
    <span className="flex gap-0.5 items-center">
      {Array.from({ length: 5 }, (_, i) => (
        <span
          key={i}
          className="inline-block w-1.5 h-1.5 rounded-full"
          style={{ background: i < value ? 'var(--color-rust)' : 'var(--color-border)' }}
        />
      ))}
    </span>
  )
}

export function RecipeCard({ recipe }: { recipe: RecipeSummary }) {
  const color = categoryColor(recipe.category)

  return (
    <Link
      to="/recipes/$id"
      params={{ id: String(recipe.id) }}
      className="group flex flex-col rounded-xl overflow-hidden transition-all duration-300"
      style={{
        background: 'white',
        border: '1px solid var(--color-border)',
        boxShadow: 'var(--shadow-card)',
      }}
      onMouseEnter={e => {
        const el = e.currentTarget as HTMLElement
        el.style.transform = 'translateY(-4px)'
        el.style.boxShadow = 'var(--shadow-card-hover)'
      }}
      onMouseLeave={e => {
        const el = e.currentTarget as HTMLElement
        el.style.transform = ''
        el.style.boxShadow = 'var(--shadow-card)'
      }}
    >
      {/* Image / placeholder */}
      <div className="relative h-44 overflow-hidden" style={{ background: `${color}18` }}>
        {recipe.cover_image ? (
          <img
            src={recipe.cover_image}
            alt={recipe.name}
            loading="lazy"
            className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          />
        ) : (
          <div
            className="w-full h-full flex items-end justify-end p-4 text-6xl select-none"
            style={{ color: `${color}30` }}
          >
            🍽
          </div>
        )}
        {/* Category badge */}
        <span
          className="absolute top-2.5 left-2.5 text-[10px] font-medium px-2 py-0.5 rounded-full"
          style={{ background: color, color: 'white' }}
        >
          {recipe.category}
        </span>
      </div>

      {/* Info */}
      <div className="p-3.5 flex flex-col gap-2">
        <h3
          className="font-semibold text-sm leading-snug line-clamp-2"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          {recipe.name}
        </h3>

        <div className="flex items-center justify-between">
          <DifficultyDots value={recipe.difficulty} />
          <div className="flex items-center gap-3 text-xs" style={{ color: 'var(--color-ink-muted)' }}>
            {recipe.calories && (
              <span className="flex items-center gap-0.5">
                <Flame size={10} style={{ color: 'var(--color-caramel)' }} />
                {Math.round(recipe.calories)}
              </span>
            )}
            <span className="flex items-center gap-0.5">
              <ChefHat size={10} />
              {recipe.ingredient_count} 材料
            </span>
          </div>
        </div>
      </div>
    </Link>
  )
}
