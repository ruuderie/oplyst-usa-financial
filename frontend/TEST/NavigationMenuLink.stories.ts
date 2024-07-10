import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuLink from '../components/ui/navigation-menu/NavigationMenuLink.vue';

const meta = {
  title: 'NavigationMenuLink',
  component: NavigationMenuLink,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuLink>;

export default meta;
type Story = StoryObj<typeof NavigationMenuLink>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};