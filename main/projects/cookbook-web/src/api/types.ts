// ── Shared ────────────────────────────────────────────────────────────────────

export interface PagedResult<T> {
  data: T[]
  page: number
  per_page: number
  total: number
  total_pages: number
}

export interface ErrorResponse {
  error: string
  message: string
}

// ── Recipe ────────────────────────────────────────────────────────────────────

export interface RecipeSummary {
  id: number
  name: string
  category: string
  difficulty: number | null
  calories: number | null
  cover_image: string | null
  source: string
  ingredient_count: number
}

export interface Ingredient {
  id: number
  recipe_id: number
  name: string
  amount: string | null
  unit: string | null
}

export interface Step {
  id: number
  recipe_id: number
  step_order: number
  content: string | null
  image_url: string | null
}

export interface Nutrition {
  id: number
  recipe_id: number
  protein_g: number | null
  fat_g: number | null
  carbs_g: number | null
  sodium_mg: number | null
}

export interface RecipeDetail {
  id: number
  name: string
  category: string
  difficulty: number | null
  calories: number | null
  cover_image: string | null
  source: string
  source_path: string
  description: string | null
  created_at: string
  tags: string[]
  ingredients: Ingredient[]
  steps: Step[]
  nutrition: Nutrition | null
}

// ── Category / Stats ──────────────────────────────────────────────────────────

export interface CategoryCount {
  name: string
  count: number
}

export interface StatsResponse {
  total_recipes: number
  avg_calories: number | null
  by_category: Record<string, number>
  sources: Record<string, number>
}

// ── Ingredient ────────────────────────────────────────────────────────────────

export interface IngredientSummary {
  name: string
  recipe_count: number
}

// ── Meal Plan ─────────────────────────────────────────────────────────────────

export interface MealPlanRequest {
  days?: number
  people?: number
  max_calories_per_meal?: number
  max_difficulty?: number
  tags?: string[]
}

export interface DayPlan {
  day: number
  breakfast: RecipeSummary
  lunch: RecipeSummary[]
  dinner: RecipeSummary[]
}

export interface MealPlanResponse {
  days: DayPlan[]
  people: number
}

// ── Query Params ──────────────────────────────────────────────────────────────

export interface RecipeListParams {
  page?: number
  per_page?: number
  category?: string
  difficulty?: number
  has_image?: boolean
  source?: string
}

export interface RecipeSearchParams {
  q: string
  page?: number
  per_page?: number
}

export interface ByIngredientsParams {
  ingredients?: string
  match?: 'any' | 'all'
  page?: number
  per_page?: number
}

export interface SimilarParams {
  limit?: number
}

export interface IngredientListParams {
  page?: number
  per_page?: number
}

export interface IngredientSuggestParams {
  q?: string
  limit?: number
}
