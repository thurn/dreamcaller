#nullable enable

using System;
using DG.Tweening;
using Dreamcaller.Layout;
using Dreamcaller.Schema;
using Dreamcaller.Services;
using Dreamcaller.Utils;
using TMPro;
using UnityEngine;

namespace Dreamcaller.Components
{
  public class Card : Displayable
  {
    [SerializeField] Transform _cardFront = null!;
    [SerializeField] TextMeshPro _name = null!;
    [SerializeField] TextMeshPro _rulesText = null!;
    [SerializeField] MeshRenderer _cardFrame = null!;
    [SerializeField] MeshRenderer _cardImage = null!;
    [SerializeField] MeshRenderer _cardBack = null!;
    [SerializeField] Material _material1 = null!;
    [SerializeField] Material _material2 = null!;
    [SerializeField] Material _material3 = null!;
    [SerializeField] Material _material4 = null!;
    [SerializeField] Material _material5 = null!;
    [SerializeField] MeshRenderer _outline = null!;
    [SerializeField] MeshRenderer _costBackground = null!;
    [SerializeField] TextMeshPro _costText = null!;
    [SerializeField] MeshRenderer _sparkBackground = null!;
    [SerializeField] TextMeshPro _sparkText = null!;

    bool _isRevealed = false;
    Registry _registry = null!;
    CardView _cardView = null!;
    Quaternion _initialDragRotation;
    float _dragStartScreenZ;
    Vector3 _dragStartPosition;
    Vector3 _dragOffset;

    public CardView CardView => Errors.CheckNotNull(_cardView);

    public void Render(Registry registry, CardView view, Sequence? sequence = null)
    {
      gameObject.name = view.Revealed?.Name ?? "Hidden Card";
      _registry = registry;
      _cardView = view;
      SortingKey = view.Position.SortingKey;

      if (view.Revealed != null)
      {
        if (_isRevealed)
        {
          RenderCardView();
        }
        else
        {
          Flip(_cardFront, _cardBack, sequence, RenderCardView);
        }
      }
      else
      {
        if (_isRevealed)
        {
          Flip(_cardBack, _cardFront, sequence, RenderCardView);
        }
        else
        {
          RenderCardView();
        }
      }
    }

    public void TurnFaceDown(Sequence? sequence = null) => Flip(_cardFront, _cardBack, sequence);

    public override bool CanHandleMouseEvents() => true;

    void Flip(Component faceUp, Component faceDown, Sequence? sequence, Action? onFlipped = null)
    {
      if (sequence != null)
      {
        const float duration = TweenUtils.FlipAnimationDurationSeconds / 2f;
        sequence
          .Insert(0, faceDown.transform.DOLocalRotate(new Vector3(0, 90, 0), duration))
          .InsertCallback(duration, () =>
          {
            faceUp.transform.localRotation = Quaternion.Euler(0, -90, 0);
            onFlipped?.Invoke();
          })
          .Insert(duration, faceUp.transform.DOLocalRotate(Vector3.zero, duration));
      }
      else
      {
        onFlipped?.Invoke();
      }
    }

    void RenderCardView()
    {
      if (CardView.Revealed != null)
      {
        RenderRevealedCardView(CardView.Revealed);
      }
      else
      {
        RenderHiddenCardView();
      }
    }

    void RenderRevealedCardView(RevealedCardView revealed)
    {
      _isRevealed = true;
      _cardFront.gameObject.SetActive(value: true);
      _cardBack.gameObject.SetActive(value: false);
      _name.text = revealed.Name;
      _rulesText.text = revealed.RulesText;
      _outline.gameObject.SetActive(CanPlay());
      _costText.text = revealed.Cost.ToString();

      if (revealed.Image?.Image.Contains("1633431262") == true)
      {
        _cardImage.material = _material1;
      }
      else if (revealed.Image?.Image.Contains("2027158310") == true)
      {
        _cardImage.material = _material2;
      }
      else if (revealed.Image?.Image.Contains("2269064809") == true)
      {
        _cardImage.material = _material3;
      }
      else if (revealed.Image?.Image.Contains("2269064817") == true)
      {
        _cardImage.material = _material4;
      }
      else if (revealed.Image?.Image.Contains("2521694543") == true)
      {
        _cardImage.material = _material5;
      }
    }

    void RenderHiddenCardView()
    {
      _isRevealed = false;
      _cardFront.gameObject.SetActive(value: false);
      _cardBack.gameObject.SetActive(value: true);
      gameObject.name = "Hidden Card";
    }

    protected override void OnSetGameContext(GameContext oldContext, GameContext newContext)
    {
      if (newContext.IsBattlefieldContext())
      {
        _cardFrame.gameObject.SetActive(false);
        _name.gameObject.SetActive(false);
        _rulesText.gameObject.SetActive(false);
      }
      else
      {
        _cardFrame.gameObject.SetActive(true);
        _name.gameObject.SetActive(true);
        _rulesText.gameObject.SetActive(true);
      }
    }

    public override void MouseDown()
    {
      if (CanPlay())
      {
        _registry.SoundService.PlayCardSound();
        GameContext = GameContext.Dragging;
        if (Parent)
        {
          Parent.RemoveIfPresent(this);
        }
        _outline.gameObject.SetActive(false);
        _initialDragRotation = transform.rotation;
        _dragStartScreenZ = Camera.main.WorldToScreenPoint(gameObject.transform.position).z;
        _dragStartPosition = _registry.InputService.WorldMousePosition(_dragStartScreenZ);
        _dragOffset = gameObject.transform.position - _dragStartPosition;
      }
    }

    public override void MouseDrag()
    {
      var mousePosition = _registry.InputService.WorldMousePosition(_dragStartScreenZ);
      var distanceDragged = Vector2.Distance(mousePosition, _dragStartPosition);
      var t = Mathf.Clamp01(distanceDragged / 2);
      transform.position = _dragOffset + mousePosition;
      var rotation = Quaternion.Slerp(_initialDragRotation, Quaternion.Euler(Constants.CameraXAngle, 0, 0), t);
      transform.rotation = rotation;

      if (distanceDragged > 0.25f)
      {
        _registry.CardService.ClearInfoZoom();
      }
    }

    public override void MouseUp()
    {
      _registry.SoundService.PlayCardSound();

      if (ShouldReturnToPreviousParentOnRelease())
      {
        _registry.LayoutUpdateService.AddToParent(this);
        _registry.LayoutUpdateService.RunAnimations(() =>
        {
          _outline.gameObject.SetActive(CanPlay());
        });
      }
      else
      {
        var action = new UserAction
        {
          BattleAction = new BattleAction
          {
            PlayCard = CardView.Id
          }
        };

        _registry.ActionService.PerformAction(action);
      }
    }

    bool ShouldReturnToPreviousParentOnRelease()
    {
      if (!CanPlay())
      {
        return true;
      }

      return !_registry.CardService.IsTouchOverPlayCardArea();
    }

    bool CanPlay() => CardView.Revealed?.CanPlay == true && _registry.CapabilitiesService.CanMoveCards();
  }
}