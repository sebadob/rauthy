{{/*
Expand the name of the chart.
*/}}
{{- define "rauthy.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "rauthy.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "rauthy.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "rauthy.labels" -}}
helm.sh/chart: {{ include "rauthy.chart" . }}
{{ include "rauthy.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "rauthy.selectorLabels" -}}
app.kubernetes.io/name: {{ include "rauthy.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}


{{/*
Check if TLS is enabled through either ingress or httpRoute
*/}}
{{- define "rauthy.isTLSEnabled" -}}
{{- if and .Values.ingress.enabled .Values.ingress.tls -}}
true
{{- else if and .Values.httpRoute.enabled .Values.httpRoute.rules -}}
{{- range .Values.httpRoute.rules -}}
{{- if .filters -}}
{{- range .filters -}}
{{- if and (eq .type "RequestHeaderModifier") .requestHeaderModifier.set -}}
{{- range .requestHeaderModifier.set -}}
{{- if and (eq .name "X-Forwarded-Proto") (eq .value "https") -}}
true
{{- end -}}
{{- end -}}
{{- end -}}
{{- end -}}
{{- end -}}
{{- end -}}
{{- else -}}
false
{{- end -}}
{{- end }}

{{/*
Get the domain name by checking ingress hosts first, then httpRoute hosts, falling back to rauthy.local
*/}}
{{- define "rauthy.domainName" -}}
{{- if and .Values.ingress.enabled .Values.ingress.hosts }}
{{- (index .Values.ingress.hosts 0).host }}
{{- else if and .Values.httpRoute.enabled .Values.httpRoute.hostnames }}
{{- index .Values.httpRoute.hostnames 0 }}
{{- else -}}
rauthy.local
{{- end }}
{{- end }}

{{/*
Get the external scheme based on TLS configuration
*/}}
{{- define "rauthy.externalScheme" -}}
{{- if eq (include "rauthy.isTLSEnabled" .) "true" -}}
https
{{- else -}}
http
{{- end -}}
{{- end }}

{{/*
Generate the public URL based on the external scheme and domain name
*/}}
{{- define "rauthy.pubUrl" -}}
{{- printf "%s://%s" (include "rauthy.externalScheme" .) (include "rauthy.domainName" .) -}}
{{- end }}

{{/*
Generate the rp_origin based on the external scheme and domain name with appropriate port
*/}}
{{- define "rauthy.rpOrigin" -}}
{{- if eq (include "rauthy.isTLSEnabled" .) "true" -}}
{{- printf "%s://%s:443" (include "rauthy.externalScheme" .) (include "rauthy.domainName" .) -}}
{{- else -}}
{{- printf "%s://%s:%d" (include "rauthy.externalScheme" .) (include "rauthy.domainName" .) (int .Values.ports.http) -}}
{{- end -}}
{{- end }}

{{/*
Get the server scheme based on port configuration
*/}}
{{- define "rauthy.apiScheme" -}}
{{- if .Values.ports.https -}}
https
{{- else -}}
http
{{- end -}}
{{- end }}
