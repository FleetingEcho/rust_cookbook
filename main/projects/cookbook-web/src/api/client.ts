const BASE = '/api/v1'

class ApiError extends Error {
  constructor(
    public status: number,
    public body: { error: string; message: string },
  ) {
    super(body.message)
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...init?.headers },
    ...init,
  })
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: 'unknown', message: res.statusText }))
    throw new ApiError(res.status, body)
  }
  return res.json()
}

function qs(params: Record<string, unknown>): string {
  const p = new URLSearchParams()
  for (const [k, v] of Object.entries(params)) {
    if (v !== undefined && v !== null && v !== '') p.set(k, String(v))
  }
  const s = p.toString()
  return s ? `?${s}` : ''
}

import type {
  PagedResult,
  RecipeSummary,
  RecipeDetail,
  RecipeListParams,
  RecipeSearchParams,
  ByIngredientsParams,
  SimilarParams,
  CategoryCount,
  StatsResponse,
  IngredientSummary,
  IngredientListParams,
  IngredientSuggestParams,
  MealPlanRequest,
  MealPlanResponse,
} from './types'

export const api = {
  // ── Recipes ────────────────────────────────────────────────────────────────
  recipes: {
    list: (p: RecipeListParams = {}) =>
      request<PagedResult<RecipeSummary>>(`/recipes${qs(p)}`),

    get: (id: number) =>
      request<RecipeDetail>(`/recipes/${id}`),

    random: (category?: string) =>
      request<RecipeDetail>(`/recipes/random${qs({ category })}`),

    similar: (id: number, p: SimilarParams = {}) =>
      request<RecipeSummary[]>(`/recipes/${id}/similar${qs(p)}`),

    search: (p: RecipeSearchParams) =>
      request<PagedResult<RecipeSummary>>(`/recipes/search${qs(p)}`),

    byIngredients: (p: ByIngredientsParams) =>
      request<PagedResult<RecipeSummary>>(`/recipes/by-ingredients${qs(p)}`),
  },

  // ── Categories / Stats ─────────────────────────────────────────────────────
  categories: {
    list: () => request<CategoryCount[]>('/categories'),
    stats: () => request<StatsResponse>('/stats'),
  },

  // ── Ingredients ────────────────────────────────────────────────────────────
  ingredients: {
    list: (p: IngredientListParams = {}) =>
      request<PagedResult<IngredientSummary>>(`/ingredients${qs(p)}`),
    suggest: (p: IngredientSuggestParams = {}) =>
      request<IngredientSummary[]>(`/ingredients/suggest${qs(p)}`),
  },

  // ── Meal Plan ──────────────────────────────────────────────────────────────
  mealPlan: {
    generate: (body: MealPlanRequest) =>
      request<MealPlanResponse>('/meal-plan', { method: 'POST', body: JSON.stringify(body) }),
  },

  // ── Image management ───────────────────────────────────────────────────────
  images: {
    upload: async (id: number, file: File): Promise<{ url: string }> => {
      const form = new FormData()
      form.append('image', file)
      const res = await fetch(`${BASE}/recipes/${id}/image`, { method: 'POST', body: form })
      if (!res.ok) {
        const body = await res.json().catch(() => ({ error: 'unknown', message: res.statusText }))
        throw new ApiError(res.status, body)
      }
      return res.json()
    },
    delete: (id: number) =>
      request<{ ok: boolean }>(`/recipes/${id}/image`, { method: 'DELETE' }),
  },
}
