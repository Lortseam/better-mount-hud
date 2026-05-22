package me.lortseam.bettermounthud.mixin;

import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.Gui;
import net.minecraft.client.multiplayer.MultiPlayerGameMode;
import net.minecraft.client.player.LocalPlayer;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.PlayerRideableJumping;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.*;

@Mixin(Gui.class)
public abstract class GuiMixin {
    @Shadow @Final private Minecraft minecraft;

    @Shadow
    private LivingEntity getPlayerVehicleWithHealth() {return null;}

    @Shadow
    private int getVehicleMaxHearts(LivingEntity entity) {return 0;}

    @ModifyVariable(method = "extractVehicleHealth", at = @At(value = "STORE"), name = "yLine1")
    private int bettermounthud$moveMountHealthUp(int y) {
        if (minecraft.gameMode.canHurtPlayer()) {
            y -= 10;
        }
        return y;
    }

    @Redirect(method = "extractPlayerHealth", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Gui;getVehicleMaxHearts(Lnet/minecraft/world/entity/LivingEntity;)I"))
    private int bettermounthud$alwaysRenderFood(Gui gui, LivingEntity entity) {
        return 0;
    }

    @ModifyVariable(method = "getAirBubbleYLine", at = @At(value = "HEAD"), ordinal = 0, argsOnly = true)
    private int bettermounthud$moveAirUp(int heartCount) {
        LivingEntity entity = getPlayerVehicleWithHealth();
        if (entity != null) {
            return getVehicleMaxHearts(entity);
        }
        return heartCount;
    }

    @Redirect(method = "nextContextualInfoState", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/LocalPlayer;jumpableVehicle()Lnet/minecraft/world/entity/PlayerRideableJumping;"))
    private PlayerRideableJumping bettermounthud$switchBar(LocalPlayer player) {
        var jumpableVehicle = player.jumpableVehicle();
        if (!minecraft.gameMode.hasExperience() || minecraft.options.keyJump.isDown()
                || player.getJumpRidingScale() > 0) return jumpableVehicle;
        return null;
    }

    @Redirect(method = "extractHotbarAndDecorations", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;hasExperience()Z"))
    private boolean bettermounthud$renderExperienceLevel(MultiPlayerGameMode gameMode) {
        return gameMode.hasExperience() &&
                ((minecraft.player.jumpableVehicle() != null
                        && !minecraft.options.keyJump.isDown()
                        && minecraft.player.getJumpRidingScale() <= 0)
                || minecraft.player.jumpableVehicle() == null);
    }
}
