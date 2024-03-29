include $(TOPDIR)/rules.mk

PKG_NAME:=alist-fuse
PKG_VERSION:=0.1.1
PKG_RELEASE:=$(AUTORELEASE)

PKG_LICENSE:=MIT
PKG_MAINTAINER:=ykxVK8yL5L <https://github.com/ykxVK8yL5L>

PKG_LIBC:=musl
ifeq ($(ARCH),arm)
  PKG_LIBC:=musleabi

  ARM_CPU_FEATURES:=$(word 2,$(subst +,$(space),$(call qstrip,$(CONFIG_CPU_TYPE))))
  ifneq ($(filter $(ARM_CPU_FEATURES),vfp vfpv2),)
    PKG_LIBC:=musleabihf
  endif
endif

PKG_ARCH=$(ARCH)
ifeq ($(ARCH),i386)
  PKG_ARCH:=i686
endif

PKG_SOURCE:=alist-fuse-v$(PKG_VERSION).$(PKG_ARCH)-unknown-linux-$(PKG_LIBC).tar.gz
PKG_SOURCE_URL:=https://github.com/ykxVK8yL5L/alist-fuse/releases/download/v$(PKG_VERSION)/
PKG_HASH:=skip

include $(INCLUDE_DIR)/package.mk

define Package/alist-fuse
  SECTION:=multimedia
  CATEGORY:=Multimedia
  DEPENDS:=+fuse-utils
  TITLE:=FUSE for alist
  URL:=https://github.com/ykxVK8yL5L/alist-fuse
endef

define Package/alist-fuse/description
  FUSE for alist.
endef

define Package/alist-fuse/conffiles
/etc/config/alist-fuse
endef

define Download/sha256sum
  FILE:=$(PKG_SOURCE).sha256
  URL_FILE:=$(FILE)
  URL:=$(PKG_SOURCE_URL)
  HASH:=skip
endef
$(eval $(call Download,sha256sum))

define Build/Prepare
	mv $(DL_DIR)/$(PKG_SOURCE).sha256 .
	cp $(DL_DIR)/$(PKG_SOURCE) .
	shasum -a 256 -c $(PKG_SOURCE).sha256
	rm $(PKG_SOURCE).sha256 $(PKG_SOURCE)

	tar -C $(PKG_BUILD_DIR)/ -zxf $(DL_DIR)/$(PKG_SOURCE)
endef

define Build/Compile
	echo "alist-fuse using precompiled binary."
endef

define Package/alist-fuse/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/alist-fuse $(1)/usr/bin/alist-fuse
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) ./files/alist-fuse.init $(1)/etc/init.d/alist-fuse
	$(INSTALL_DIR) $(1)/etc/config
	$(INSTALL_CONF) ./files/alist-fuse.config $(1)/etc/config/alist-fuse
endef

$(eval $(call BuildPackage,alist-fuse))
