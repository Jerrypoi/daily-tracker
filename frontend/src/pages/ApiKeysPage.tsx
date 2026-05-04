import { useEffect, useState } from 'react'
import type { FormEvent } from 'react'

import { getErrorMessage } from '../api/errors'
import { createApiKey, listApiKeys, revokeApiKey } from '../api/apiKeys'
import type { ApiKey, CreateApiKeyResponse } from '../api/apiKeys'

function formatDate(value: string | undefined | null): string {
  if (!value) return 'never'
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value
  return date.toLocaleString()
}

export function ApiKeysPage() {
  const [keys, setKeys] = useState<ApiKey[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const [name, setName] = useState('')
  const [creating, setCreating] = useState(false)
  const [createError, setCreateError] = useState<string | null>(null)
  const [createdKey, setCreatedKey] = useState<CreateApiKeyResponse | null>(null)
  const [copied, setCopied] = useState(false)

  const [revokingId, setRevokingId] = useState<number | null>(null)

  async function loadKeys() {
    setLoading(true)
    setError(null)
    try {
      const data = await listApiKeys()
      setKeys(data)
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    void loadKeys()
  }, [])

  async function onCreate(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setCreateError(null)

    const trimmed = name.trim()
    if (!trimmed) {
      setCreateError('name is required')
      return
    }

    setCreating(true)
    try {
      const created = await createApiKey(trimmed)
      setCreatedKey(created)
      setCopied(false)
      setName('')
      await loadKeys()
    } catch (err) {
      setCreateError(getErrorMessage(err))
    } finally {
      setCreating(false)
    }
  }

  async function onRevoke(key: ApiKey) {
    const confirmed = window.confirm(
      `Revoke API key "${key.name}"? Anything using this key will stop working immediately.`,
    )
    if (!confirmed) return

    setRevokingId(key.id)
    try {
      await revokeApiKey(key.id)
      await loadKeys()
    } catch (err) {
      setError(getErrorMessage(err))
    } finally {
      setRevokingId(null)
    }
  }

  async function copyToken() {
    if (!createdKey) return
    try {
      await navigator.clipboard.writeText(createdKey.token)
      setCopied(true)
    } catch {
      setCopied(false)
    }
  }

  return (
    <section className="page">
      <h2>API Keys</h2>

      <form className="panel" onSubmit={onCreate}>
        <h3>Create API Key</h3>
        <p className="modal-meta">
          Use API keys to call the API from scripts or other clients. Pass them
          in the <code>Authorization: Bearer &lt;token&gt;</code> header, just
          like a JWT.
        </p>
        <label>
          Name
          <input
            required
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="ci-bot"
          />
        </label>
        <button type="submit" disabled={creating}>
          {creating ? 'Creating...' : 'Create API Key'}
        </button>
        {createError && <p className="error">{createError}</p>}
      </form>

      <div className="panel">
        <h3>Active Keys</h3>
        {loading && <p>Loading...</p>}
        {error && <p className="error">{error}</p>}
        {!loading && !error && (
          keys.length === 0 ? (
            <ul className="list"><li>No API keys yet.</li></ul>
          ) : (
            <ul className="list">
              {keys.map((key) => (
                <li key={key.id} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: '12px' }}>
                  <div>
                    <strong>{key.name}</strong>
                    <div style={{ fontSize: '0.85em', color: '#666' }}>
                      <code>{key.key_prefix}…</code>
                      {' • '}created {formatDate(key.created_at)}
                      {' • '}last used {formatDate(key.last_used_at)}
                    </div>
                  </div>
                  <button
                    type="button"
                    onClick={() => onRevoke(key)}
                    disabled={revokingId === key.id}
                    style={{ background: '#ef4444', color: 'white', border: 'none', borderRadius: '4px', padding: '6px 12px', cursor: 'pointer' }}
                  >
                    {revokingId === key.id ? 'Revoking...' : 'Revoke'}
                  </button>
                </li>
              ))}
            </ul>
          )
        )}
      </div>

      {createdKey && (
        <div className="modal-backdrop" role="presentation" onClick={() => setCreatedKey(null)}>
          <div
            className="modal-panel"
            role="dialog"
            aria-modal="true"
            onClick={(event) => event.stopPropagation()}
          >
            <h3>Save your API key</h3>
            <p className="modal-meta">
              This is the only time the full token will be shown. Copy it now —
              once you close this dialog it cannot be retrieved.
            </p>
            <pre style={{ background: '#f3f4f6', padding: '12px', borderRadius: '4px', overflowX: 'auto', userSelect: 'all' }}>
              {createdKey.token}
            </pre>
            <div className="modal-actions">
              <button type="button" onClick={copyToken}>
                {copied ? 'Copied!' : 'Copy to clipboard'}
              </button>
              <button type="button" onClick={() => setCreatedKey(null)}>
                I have saved it
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  )
}
