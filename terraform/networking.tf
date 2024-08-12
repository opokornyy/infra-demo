resource "kubernetes_service" "app_service" {
  metadata {
    name      = "app-service"
    namespace = kubernetes_namespace.app.metadata[0].name
  }

  spec {
    selector = {
      app = "app"
    }

    type = "ClusterIP"

    port {
      port = var.container_ports["app"]
    }
  }
}

resource "kubernetes_service" "db_service" {
  metadata {
    name      = "db"
    namespace = kubernetes_namespace.app.metadata[0].name
    labels = {
      app = "app-db"
    }
  }

  spec {
    selector = {
      app = "app-db"
    }

    type = "ClusterIP"

    port {
      protocol    = "TCP"
      port        = var.container_ports["db"]
      target_port = var.container_ports["db"]
    }
  }
}

// V1 needs to be used because of some issues with the current version of the provider
resource "kubernetes_ingress_v1" "app_ingress" {
  metadata {
    name      = "app-ingress"
    namespace = kubernetes_namespace.app.metadata[0].name
    labels = {
      app = "app"
    }
    annotations = {
      "nginx.ingress.kubernetes.io/rewrite-target" = "/"
    }
  }

  spec {
    tls {
      hosts       = ["web-app.local"]
      secret_name = "app-secret"
    }

    rule {
      host = "web-app.local"
      http {
        path {
          path      = "/"
          path_type = "Prefix"

          backend {
            service {
              name = kubernetes_service.app_service.metadata[0].name
              port {
                number = kubernetes_service.app_service.spec[0].port[0].port
              }
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "static_web_service" {
  metadata {
    name      = "static-web-service"
    namespace = kubernetes_namespace.app.metadata[0].name
  }

  spec {
    selector = {
      app = "static-web"
    }

    type = "ClusterIP"

    port {
      port        = var.container_ports["app"]
      target_port = var.container_ports["app"]
    }
  }
}

resource "kubernetes_ingress_v1" "static_web_ingress" {
  metadata {
    name      = "static-web-ingress"
    namespace = kubernetes_namespace.app.metadata[0].name
    labels = {
      app = "static-web"
    }
    annotations = {
      "nginx.ingress.kubernetes.io/rewrite-target" = "/"
    }
  }

  spec {
    tls {
      hosts       = ["static-web.local"]
      secret_name = "app-secret"
    }

    rule {
      host = "static-web.local"
      http {
        path {
          path      = "/"
          path_type = "Prefix"

          backend {
            service {
              name = kubernetes_service.static_web_service.metadata[0].name
              port {
                number = var.container_ports["app"]
              }
            }
          }
        }
      }
    }
  }
}
