import { useQuery, useMutation, useQueryClient, keepPreviousData } from '@tanstack/react-query'
import { api } from './client'
import type {
  RecipeListParams,
  RecipeSearchParams,
  ByIngredientsParams,
  SimilarParams,
  IngredientListParams,
  IngredientSuggestParams,
  MealPlanRequest,
} from './types'

const STALE = 60_000

export function useRecipes(p: RecipeListParams = {}) {
  return useQuery({
    queryKey: ['recipes', p],
    queryFn: () => api.recipes.list(p),
    staleTime: STALE,
    placeholderData: keepPreviousData,
  })
}

export function useRecipe(id: number) {
  return useQuery({
    queryKey: ['recipe', id],
    queryFn: () => api.recipes.get(id),
    staleTime: STALE,
    enabled: id > 0,
  })
}

export function useRandomRecipe(category?: string) {
  return useQuery({
    queryKey: ['recipe', 'random', category],
    queryFn: () => api.recipes.random(category),
    staleTime: 0,
  })
}

export function useSimilarRecipes(id: number, p: SimilarParams = {}) {
  return useQuery({
    queryKey: ['recipes', 'similar', id, p],
    queryFn: () => api.recipes.similar(id, p),
    staleTime: STALE,
    enabled: id > 0,
  })
}

export function useRecipeSearch(p: RecipeSearchParams) {
  return useQuery({
    queryKey: ['recipes', 'search', p],
    queryFn: () => api.recipes.search(p),
    staleTime: STALE,
    placeholderData: keepPreviousData,
    enabled: p.q.length >= 1,
  })
}

export function useByIngredients(p: ByIngredientsParams) {
  const enabled = (p.ingredients ?? '').trim().length > 0
  return useQuery({
    queryKey: ['recipes', 'by-ingredients', p],
    queryFn: () => api.recipes.byIngredients(p),
    staleTime: STALE,
    placeholderData: keepPreviousData,
    enabled,
  })
}

export function useCategories() {
  return useQuery({
    queryKey: ['categories'],
    queryFn: api.categories.list,
    staleTime: 5 * 60_000,
  })
}

export function useStats() {
  return useQuery({
    queryKey: ['stats'],
    queryFn: api.categories.stats,
    staleTime: 5 * 60_000,
  })
}

export function useIngredients(p: IngredientListParams = {}) {
  return useQuery({
    queryKey: ['ingredients', p],
    queryFn: () => api.ingredients.list(p),
    staleTime: STALE,
    placeholderData: keepPreviousData,
  })
}

export function useIngredientSuggest(p: IngredientSuggestParams) {
  return useQuery({
    queryKey: ['ingredients', 'suggest', p],
    queryFn: () => api.ingredients.suggest(p),
    staleTime: STALE,
    enabled: (p.q ?? '').length >= 1,
  })
}

export function useMealPlan() {
  return useMutation({
    mutationFn: (body: MealPlanRequest) => api.mealPlan.generate(body),
  })
}

export function useUploadImage(id: number) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: (file: File) => api.images.upload(id, file),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['recipe', id] }),
  })
}

export function useDeleteImage(id: number) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: () => api.images.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['recipe', id] }),
  })
}
