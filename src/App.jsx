import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  RefreshCw,
  Server,
  CircleCheck,
  CircleX,
  Globe,
  Settings,
  Database,
  Code,
} from "lucide-react";
import "./App.css";

function App() {
  const [sites, setSites] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    loadSites();
  }, []);

  async function loadSites() {
    try {
      setLoading(true);
      setError(null);
      const siteList = await invoke("get_sites");
      setSites(siteList);
    } catch (err) {
      console.error("Failed to load sites:", err);
      setError(err.toString());
    } finally {
      setLoading(false);
    }
  }

  const getStatusColor = (status) => {
    return status === "UP" ? "#e3efe7" : "#fbe1e1";
  };

  const getSiteUrl = (siteUrl) => {
    return siteUrl; // Direct site URL
  };

  const getWpAdminUrl = (siteUrl) => {
    return `${siteUrl}/wp-admin`; // WordPress admin URL
  };

  const getPhpMyAdminUrl = (siteUrl) => {
    // Add pma. subdomain to the site domain
    const url = new URL(siteUrl);
    url.hostname = `pma.${url.hostname}`;
    return url.toString();
  };

  const openInVSCode = async (siteName) => {
    try {
      await invoke("open_in_vscode", { siteName });
    } catch (err) {
      console.error("Failed to open VS Code:", err);
    }
  };

  const toggleSite = async (siteName, currentStatus) => {
    try {
      await invoke("toggle_site", { siteName, currentStatus });
      // Refresh the sites list after toggling
      setTimeout(() => {
        loadSites();
      }, 1000); // Wait a bit for the command to complete
    } catch (err) {
      console.error("Failed to toggle site:", err);
    }
  };

  return (
    <main className="container">
      <div className="header">
        <h1>Sites</h1>
        <button onClick={loadSites} disabled={loading} className="refresh-btn">
          <RefreshCw size={10} className={loading ? "spinning" : ""} />
          {loading ? "Loading..." : "Refresh"}
        </button>
      </div>

      {error && (
        <div className="error">
          <strong>Error:</strong> {error}
        </div>
      )}

      {loading && <div className="loading">Loading sites...</div>}

      {!loading && !error && (
        <div className="sites-grid">
          {sites.length === 0 ? (
            <div className="no-sites">
              <p>
                No sites found. Use 'sail site add &lt;name&gt;' to create a new
                site.
              </p>
            </div>
          ) : (
            sites.map((site) => (
              <div key={site.name} className="site-card">
                <div className="site-header">
                  <div className="site-title">
                    <h3>{site.name}</h3>
                  </div>
                  <button
                    className={`status-badge ${
                      site.status === "UP" ? "up" : "down"
                    }`}
                    style={{ backgroundColor: getStatusColor(site.status) }}
                    onClick={() => toggleSite(site.name, site.status)}
                    title={`Click to ${
                      site.status === "UP" ? "stop" : "start"
                    } site`}
                  >
                    {site.status === "UP" ? (
                      <CircleCheck size={12} />
                    ) : (
                      <CircleX size={12} />
                    )}
                    {site.status}
                  </button>
                </div>
                <div className="site-actions">
                  <a
                    href={getSiteUrl(site.url)}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="action-icon"
                    title="Visit Site"
                  >
                    <Globe size={14} />
                  </a>
                  <a
                    href={getWpAdminUrl(site.url)}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="action-icon"
                    title="WordPress Admin"
                  >
                    <Settings size={14} />
                  </a>
                  <a
                    href={getPhpMyAdminUrl(site.url)}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="action-icon"
                    title="phpMyAdmin"
                  >
                    <Database size={14} />
                  </a>
                  <button
                    onClick={() => openInVSCode(site.name)}
                    className="action-icon"
                    title="Open in VS Code"
                  >
                    <Code size={14} />
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      )}
    </main>
  );
}

export default App;
