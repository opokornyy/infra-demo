resource "kubernetes_service" "app_service" {
  metadata {
    name      = "app-service"
    namespace = kubernetes_namespace.app.metadata[0].name
  }

  spec {
    selector = {
      app = "app"
    }

    type = "NodePort"

    port {
      port      = 80
      node_port = 30001
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
      port        = 3306
      target_port = 3306
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
